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
use tower_http::cors::CorsLayer;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;

use error::AppError;
use state::AwsState;

use common::http::*;
//use common::messages::Message;

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

    let user = itchio::get_user(request.access_token).await?.into();

    info!("authenticated user: {:?}", user);

    aws::save_user(aws_state.get_config(), &user).await?;

    let secret = aws::get_jwt_secret(aws_state.get_config()).await?;
    let token = auth::generate_token_for_user(user.get_username(), secret)?;

    let response = AuthenticateResponse { token };
    Ok((StatusCode::OK, Json(response)))
}

async fn get_characters(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(aws_state): State<AwsState>,
) -> Result<(StatusCode, Json<GetCharactersResponse>), AppError> {
    let secret = aws::get_jwt_secret(aws_state.get_config()).await?;
    let username = auth::validate_token(bearer.token(), secret)?;

    info!("getting characters for {}", username);

    let response = GetCharactersResponse {};
    Ok((StatusCode::OK, Json(response)))
}

async fn create_duel(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(aws_state): State<AwsState>,
    Json(request): Json<CreateDuelRequest>,
) -> Result<(StatusCode, Json<CreateDuelResponse>), AppError> {
    let secret = aws::get_jwt_secret(aws_state.get_config()).await?;
    let username = auth::validate_token(bearer.token(), secret)?;

    //let user = aws::get_user(aws_state.get_config(), user_id);

    info!("creating duel for {}:{}", username, request.character_id);

    let _opponent_user_id = 1234;
    let _opponent_character_id = Uuid::new_v4();
    /*let message = Message::new_duel(
        request.user_id,
        request.character_id,
        opponent_user_id,
        opponent_character_id,
    );

    aws::post_message(aws_state.get_config(), aws_state.get_queue_url(), message).await?;*/

    let response = CreateDuelResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}
