use std::sync::Arc;

use crate::server::routering::ISubRouter;
use crate::user::model::{IUserDB, UserInput};
use crate::user::usege::UserUsage;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Router, routing::get};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState<T: IUserDB> {
    pub usage: Arc<UserUsage<T>>,
}

async fn get_users<T: IUserDB>(State(app): State<AppState<T>>) -> impl IntoResponse
where
{
    match app.usage.get_users().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update_user<T: IUserDB>(
    Path(id): Path<Uuid>,
    State(app): State<AppState<T>>,
    Json(input): Json<UserInput>,
) -> impl IntoResponse {
    match app.usage.edit_user(id, input).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn get_user<T: IUserDB>(
    Path(id): Path<Uuid>,
    State(app): State<AppState<T>>,
) -> impl IntoResponse {
    match app.usage.get_user(id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_user<T: IUserDB>(
    State(app): State<AppState<T>>,
    Json(input): Json<UserInput>,
) -> impl IntoResponse {
    match app.usage.create_user(input).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_user<T: IUserDB>(
    Path(id): Path<Uuid>,
    State(app): State<AppState<T>>,
) -> impl IntoResponse {
    match app.usage.delete_user(id).await {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub struct UserHandler<T: IUserDB> {
    apps: AppState<T>,
}

impl<T: IUserDB> UserHandler<T> {
    pub fn new(usage: UserUsage<T>) -> Self {
        return Self {
            apps: AppState {
                usage: Arc::new(usage),
            },
        };
    }
}

impl<T: IUserDB> ISubRouter for UserHandler<T> {
    fn path(&self) -> &'static str {
        "/users"
    }

    fn route(&self) -> Router {
        return Router::new()
            .route("/", get(get_users).post(create_user))
            .route("/{id}", get(get_user).put(update_user).delete(delete_user))
            .with_state(self.apps.clone());
    }
}

async fn whatsup_bro() -> &'static str {
    "SHhhh.. 🤫 Here We Are Dealing With The Users 😎🔥 .. Do You Wanna Hack Them .. Shhhh 🤫 .. "
}
