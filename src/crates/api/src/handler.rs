use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use crate::app_state::AppState;
use command::command::{create_circle, update_circle};
use query::query::get_circle;
use serde::Deserialize;
use std::env;

pub async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// create
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleRequestBody {
    pub circle_name: String,
    pub capacity: i16,
}

impl std::convert::From<CreateCircleRequestBody> for create_circle::Input {
    fn from(
        CreateCircleRequestBody {
            circle_name,
            capacity,
        }: CreateCircleRequestBody,
    ) -> Self {
        create_circle::Input {
            circle_name,
            capacity,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleResponseBody {
    pub circle_id: String,
}

impl std::convert::From<create_circle::Output> for CreateCircleResponseBody {
    fn from(create_circle::Output { circle_id }: create_circle::Output) -> Self {
        CreateCircleResponseBody { circle_id }
    }
}

pub async fn handle_create_circle(
    State(state): State<AppState>,
    Json(body): Json<CreateCircleRequestBody>,
) -> Result<Json<CreateCircleResponseBody>, StatusCode> {
    match state.command_handler.create_circle(body.into()).await {
        Ok(output) => Ok(Json(CreateCircleResponseBody::from(output))),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            match e {
                create_circle::Error::InvalidInput => Err(StatusCode::BAD_REQUEST),
                create_circle::Error::Duplicate => Err(StatusCode::CONFLICT),
                create_circle::Error::Circle => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

// fetch
#[derive(Debug, Deserialize)]
pub struct FetchCircleInputParam {
    id: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FetcheCircleResponseBody {
    pub circle_id: String,
    pub circle_name: String,
    pub capacity: i16,
}

impl std::convert::From<get_circle::Output> for FetcheCircleResponseBody {
    fn from(output: get_circle::Output) -> Self {
        match output.0 {
            Some(circle) => FetcheCircleResponseBody {
                circle_id: circle.id.to_string(),
                circle_name: circle.name,
                capacity: circle.capacity,
            },
            // TODO: None の場合の処理
            None => FetcheCircleResponseBody {
                circle_id: "".to_string(),
                circle_name: "".to_string(),
                capacity: 0,
            },
        }
    }
}

// TODO: impl From<list_circles::Output> for FetcheCircleResponseBody

pub async fn handle_fetch_circle(
    State(state): State<AppState>,
    Path(param): Path<FetchCircleInputParam>,
) -> Result<Json<Vec<FetcheCircleResponseBody>>, String> {
    match param.id {
        Some(id) => {
            match state
                .query_handler
                .get_circle(get_circle::Input {
                    circle_id: id.clone(),
                })
                .await
            {
                Ok(output) => Ok(Json(vec![FetcheCircleResponseBody::from(output)])),
                Err(e) => {
                    tracing::error!("error: {:?}", e);
                    Err("error".to_string())
                }
            }
        }
        None => match state.query_handler.list_circles().await {
            Ok(output) => {
                let circles = output.0;
                Ok(Json(
                    circles
                        .into_iter()
                        .map(|circle| {
                            FetcheCircleResponseBody::from(get_circle::Output(Some(circle)))
                        })
                        .collect(),
                ))
            }
            Err(e) => {
                tracing::error!("error: {:?}", e);
                Err("error".to_string())
            }
        },
    }
}

// update
#[derive(Debug, Deserialize)]
pub struct UpdateCircleInputParam {
    id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdateCircleRequestBody {
    pub circle_name: Option<String>,
    pub capacity: Option<i16>,
    pub version: u32,
}

impl UpdateCircleRequestBody {
    pub fn into_to_input(self, id: String) -> update_circle::Input {
        update_circle::Input {
            circle_id: id,
            circle_name: self.circle_name,
            capacity: self.capacity,
            version: self.version,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateCircleResponseBody {
    pub circle_id: String,
}

impl std::convert::From<update_circle::Output> for UpdateCircleResponseBody {
    fn from(output: update_circle::Output) -> Self {
        UpdateCircleResponseBody {
            circle_id: output.circle_id,
        }
    }
}

pub async fn handle_update_circle(
    State(state): State<AppState>,
    Path(path): Path<UpdateCircleInputParam>,
    Json(body): Json<UpdateCircleRequestBody>,
) -> Result<Json<UpdateCircleResponseBody>, StatusCode> {
    tracing::info!("update circle: {:?}", body);
    let input = body.into_to_input(path.id);
    match state.command_handler.update_circle(input).await {
        Ok(output) => Ok(Json(UpdateCircleResponseBody::from(output))),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            match e {
                update_circle::Error::InvalidInput => Err(StatusCode::BAD_REQUEST),
                update_circle::Error::Duplicate => Err(StatusCode::BAD_REQUEST),
                update_circle::Error::Circle => Err(StatusCode::INTERNAL_SERVER_ERROR),
                update_circle::Error::VersionMismatch => Err(StatusCode::CONFLICT),
            }
        }
    }
}
