#![deny(warnings)]

mod auth;
mod aws;
mod error;
mod itchio;
mod state;
mod user;

use std::net::SocketAddr;

use axum::{
    extract::State,
    headers::authorization::{Authorization, Bearer},
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router, TypedHeader,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

use error::AppError;
use state::AwsState;
use user::User;

use common::http::*;
use common::messages::*;

fn init_logging() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;

    let aws_config = aws_config::load_from_env().await;

    let queue_url = aws::get_queue_url(&aws_config).await?;
    debug!("queue URL: {}", queue_url);

    let aws_state = AwsState::new(aws_config, queue_url);

    let app = Router::new()
        .route("/authenticate", post(authenticate))
        .route("/characters", get(get_characters))
        .route("/duel", post(create_duel))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers([axum::http::header::CONTENT_TYPE])
                .allow_methods([Method::OPTIONS, Method::HEAD, Method::GET, Method::POST]),
        )
        .layer(TraceLayer::new_for_http())
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

async fn authenticate(
    State(aws_state): State<AwsState>,
    Json(request): Json<AuthenticateRequest>,
) -> Result<(StatusCode, Json<AuthenticateResponse>), AppError> {
    info!("authenticating user ...");

    let mut user: User = itchio::get_user(&request.oauth_token).await?.into();
    info!("authenticated user: {}", user);
    user.set_api_key(request.oauth_token);

    aws::save_user(aws_state.get_config(), user.clone()).await?;

    let secret = aws::get_jwt_secret(aws_state.get_config()).await?;
    let access_token = auth::generate_token_for_user(user.get_user_id().to_string(), secret)?;

    let response = AuthenticateResponse {
        access_token,
        refresh_token: String::default(),
        display_name: user.get_display_name().clone(),
    };
    Ok((StatusCode::OK, Json(response)))
}

async fn get_characters(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(aws_state): State<AwsState>,
) -> Result<(StatusCode, Json<GetCharactersResponse>), AppError> {
    let secret = aws::get_jwt_secret(aws_state.get_config()).await?;
    let user_id = auth::validate_token(bearer.token(), secret)?;

    let user = aws::get_user(aws_state.get_config(), user_id.parse()?)
        .await?
        .ok_or_else(|| anyhow::anyhow!("no such user"))?;

    info!("getting characters for {}", user.get_user_id());

    let response = GetCharactersResponse {
        // ...
    };
    Ok((StatusCode::OK, Json(response)))
}

async fn create_duel(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(aws_state): State<AwsState>,
    Json(request): Json<CreateDuelRequest>,
) -> Result<(StatusCode, Json<CreateDuelResponse>), AppError> {
    let secret = aws::get_jwt_secret(aws_state.get_config()).await?;
    let user_id = auth::validate_token(bearer.token(), secret)?;

    let user = aws::get_user(aws_state.get_config(), user_id.parse()?)
        .await?
        .ok_or_else(|| anyhow::anyhow!("no such user"))?;

    info!(
        "creating duel for {}:{}",
        user.get_user_id(),
        request.character_id
    );

    let opponent_user_id = "1234";
    let opponent_character_id = Uuid::new_v4();
    let message = Message::new_duel(
        user.get_user_id(),
        request.character_id,
        opponent_user_id,
        opponent_character_id,
    );

    aws::post_message(aws_state.get_config(), aws_state.get_queue_url(), message).await?;

    let response = CreateDuelResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}
