#![deny(warnings)]

mod auth;
mod aws;
mod config;
mod error;
mod itchio;
mod notifs;
mod state;
mod user;

use std::net::SocketAddr;

use axum::{
    debug_handler,
    extract::{ws::WebSocketUpgrade, State},
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
use state::AppState;
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

    // TODO: this should be reloadable
    info!("loading config ...");
    let client_config = aws::get_client_config(&aws_config)
        .await?
        .ok_or_else(|| anyhow::anyhow!("missing client config"))?;
    info!("got client config: {:?}", client_config);

    let app_state = AppState::new(aws_config, client_config);

    let app = Router::new()
        .route("/authenticate", post(authenticate))
        .route("/refresh", post(refresh))
        .route("/notifs", get(notifs))
        .route("/config/client", get(get_client_config))
        .route("/user", get(get_user))
        .route("/characters", get(get_characters))
        .route("/duel", post(create_duel))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_headers([axum::http::header::CONTENT_TYPE])
                .allow_methods([Method::OPTIONS, Method::HEAD, Method::GET, Method::POST]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let app = app.fallback(handler_404);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[debug_handler]
async fn handler_404() -> impl IntoResponse {
    debug!("invalid resource");
    (StatusCode::NOT_FOUND, "resource not found")
}

// TODO: use auth header instead of POST
#[debug_handler]
async fn authenticate(
    State(app_state): State<AppState>,
    Json(request): Json<AuthenticateRequest>,
) -> Result<(StatusCode, Json<AuthenticateResponse>), AppError> {
    info!("authenticating user ...");

    let mut user: User = itchio::get_user(&request.oauth_token).await?.into();
    info!("authenticated user: {}", user);
    user.set_api_key(request.oauth_token);

    let aws_config = app_state.get_aws_config();
    aws::save_user(aws_config, user.clone()).await?;

    let secret = aws::get_jwt_secret(aws_config).await?;
    let (access_token, refresh_token) =
        auth::generate_tokens_for_user(user.get_user_id().to_string(), secret)?;

    let response = AuthenticateResponse {
        access_token,
        refresh_token,
    };
    Ok((StatusCode::OK, Json(response)))
}

// TODO: use auth header instead of POST
#[debug_handler]
async fn refresh(
    State(app_state): State<AppState>,
    Json(request): Json<RefreshRequest>,
) -> Result<(StatusCode, Json<AuthenticateResponse>), AppError> {
    let aws_config = app_state.get_aws_config();
    let secret = aws::get_jwt_secret(aws_config).await?;
    let user_id = auth::validate_user_refresh_token(request.refresh_token, &secret)?;

    info!("refreshing user token for {} ...", user_id);

    let (access_token, refresh_token) = auth::generate_tokens_for_user(user_id, secret)?;

    let response = AuthenticateResponse {
        access_token,
        refresh_token,
    };
    Ok((StatusCode::OK, Json(response)))
}

#[debug_handler]
async fn notifs(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse, AppError> {
    let aws_config = app_state.get_aws_config();
    let user_id = user::validate_user(aws_config, bearer.token()).await?;

    info!("{} subscribing to notifications ...", user_id);

    Ok(ws.on_upgrade(|socket| notifs::handle_notifs(socket, user_id)))
}

#[debug_handler]
async fn get_client_config(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<GetClientConfigResponse>), AppError> {
    let aws_config = app_state.get_aws_config();
    user::validate_user(aws_config, bearer.token()).await?;

    let response = GetClientConfigResponse {
        max_characters: app_state.get_client_config().max_characters,
        max_character_name_len: app_state.get_client_config().max_character_name_len,
    };
    Ok((StatusCode::OK, Json(response)))
}

#[debug_handler]
async fn get_user(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<GetUserResponse>), AppError> {
    let aws_config = app_state.get_aws_config();
    let user = user::get_user(aws_config, bearer.token()).await?;

    let response = GetUserResponse {
        user_id: user.get_user_id().clone(),
        display_name: user.get_display_name().clone(),
    };
    Ok((StatusCode::OK, Json(response)))
}

#[debug_handler]
async fn get_characters(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<GetCharactersResponse>), AppError> {
    let aws_config = app_state.get_aws_config();
    let user_id = user::validate_user(aws_config, bearer.token()).await?;

    info!("getting characters for {}", user_id);

    let response = GetCharactersResponse {
        // ...
    };
    Ok((StatusCode::OK, Json(response)))
}

#[debug_handler]
async fn create_duel(
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    State(app_state): State<AppState>,
    Json(request): Json<CreateDuelRequest>,
) -> Result<(StatusCode, Json<CreateDuelResponse>), AppError> {
    let aws_config = app_state.get_aws_config();
    let user_id = user::validate_user(aws_config, bearer.token()).await?;

    info!("creating duel for {}:{}", user_id, request.character_id);

    let opponent_user_id = "1234";
    let opponent_character_id = Uuid::new_v4();
    let message = Message::new_duel(
        user_id,
        request.character_id,
        opponent_user_id,
        opponent_character_id,
    );

    let queue_url = aws::get_queue_url(aws_config).await?;
    aws::post_message(aws_config, queue_url, message).await?;

    let response = CreateDuelResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}
