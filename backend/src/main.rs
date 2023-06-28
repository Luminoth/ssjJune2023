#![deny(warnings)]

mod error;
mod state;

use std::net::SocketAddr;

use aws_config::SdkConfig;
use axum::{
    extract::{Path, State},
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tower_http::cors::CorsLayer;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

use error::AppError;
use state::AwsState;

use common::http::*;
use common::messages::Message;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

async fn get_queue_url(aws_config: &SdkConfig) -> anyhow::Result<String> {
    info!("getting queue URL ...");

    let client = aws_sdk_sqs::Client::new(aws_config);
    let result = client.get_queue_url().queue_name("ssj2023").send().await?;
    Ok(result
        .queue_url()
        .ok_or(anyhow::anyhow!("missing queue URL"))?
        .to_owned())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;

    let aws_config = aws_config::load_from_env().await;

    let queue_url = get_queue_url(&aws_config).await?;
    debug!("queue URL: {}", queue_url);

    let aws_state = AwsState::new(aws_config, queue_url);

    let app = Router::new()
        .route("/authenticate", post(authenticate))
        .route("/characters/:id", get(get_characters))
        .route("/duel", post(create_duel))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers([axum::http::header::CONTENT_TYPE])
                .allow_methods([Method::OPTIONS, Method::HEAD, Method::GET, Method::POST]),
        )
        .with_state(aws_state);

    let app = app.fallback(handler_404);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    debug!("invalid resource");
    (StatusCode::NOT_FOUND, "resource not found")
}

async fn authenticate(Json(request): Json<AuthenticateRequest>) -> Result<(), AppError> {
    info!("authenticating user ...");

    let response = reqwest::get(format!(
        "https://itch.io/api/1/{}/credentials/info",
        request.access_token
    ))
    .await?
    .text()
    .await?;

    info!("{}", response);

    Ok(())
}

async fn get_characters(
    Path(user_id): Path<Uuid>,
) -> Result<(StatusCode, Json<GetCharactersResponse>), AppError> {
    info!("getting characters for {}", user_id);

    let response = GetCharactersResponse {};
    Ok((StatusCode::OK, Json(response)))
}

async fn create_duel(
    State(aws_state): State<AwsState>,
    Json(request): Json<CreateDuelRequest>,
) -> Result<(StatusCode, Json<CreateDuelResponse>), AppError> {
    info!(
        "creating duel for {}:{}",
        request.user_id, request.character_id
    );

    let opponent_user_id = Uuid::new_v4();
    let opponent_character_id = Uuid::new_v4();
    let message = Message::new_duel(
        request.user_id,
        request.character_id,
        opponent_user_id,
        opponent_character_id,
    );

    let client = aws_sdk_sqs::Client::new(aws_state.get_config());
    client
        .send_message()
        .queue_url(aws_state.get_queue_url())
        .message_body(message)
        .send()
        .await?;

    let response = CreateDuelResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}
