use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use command::command::{
    create_circle::{self, Input, Output},
    update_circle,
};
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
    pub owner_name: String,
    pub owner_age: i16,
    pub owner_grade: i16,
    pub owner_major: String,
}

impl std::convert::From<CreateCircleRequestBody> for create_circle::Input {
    fn from(
        CreateCircleRequestBody {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }: CreateCircleRequestBody,
    ) -> Self {
        Input {
            circle_name,
            capacity,
            owner_name,
            owner_age,
            owner_grade,
            owner_major,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CreateCircleResponseBody {
    pub circle_id: String,
    pub owner_id: String,
}

impl std::convert::From<create_circle::Output> for CreateCircleResponseBody {
    fn from(
        Output {
            circle_id,
            owner_id,
        }: Output,
    ) -> Self {
        CreateCircleResponseBody {
            circle_id,
            owner_id,
        }
    }
}

pub async fn handle_create_circle(
    State(state): State<AppState>,
    Json(body): Json<CreateCircleRequestBody>,
) -> Result<Json<CreateCircleResponseBody>, StatusCode> {
    let input = Input::from(body);
    match state.command_handler.create_circle(input).await {
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
    pub owner: MemberOutput,
    pub members: Vec<MemberOutput>,
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct MemberOutput {
    pub id: String,
    pub name: String,
    pub age: i16,
    pub grade: i16,
    pub major: String,
}

impl std::convert::From<get_circle::Output> for FetcheCircleResponseBody {
    fn from(output: get_circle::Output) -> Self {
        // FIXME: expect("fixme")
        let circle = output.0.expect("fixme");
        let owner = circle.owner;
        let members = circle.members;
        FetcheCircleResponseBody {
            circle_id: circle.id.to_string(),
            circle_name: circle.name,
            capacity: circle.capacity,
            owner: MemberOutput {
                id: owner.id.to_string(),
                name: owner.name,
                age: owner.age,
                grade: owner.grade.into(),
                major: owner.major.into(),
            },
            members: members
                .into_iter()
                .map(|member| MemberOutput {
                    id: member.id.to_string(),
                    name: member.name,
                    age: member.age,
                    grade: member.grade.into(),
                    major: member.major.into(),
                })
                .collect(),
        }
    }
}

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
                let circle = circles.first().unwrap();
                match state
                    .query_handler
                    .get_circle(get_circle::Input {
                        circle_id: circle.id.to_string(),
                    })
                    .await
                {
                    Ok(output) => {
                        let mut res = Vec::new();
                        res.push(FetcheCircleResponseBody::from(output));
                        Ok(Json(res))
                    }
                    Err(e) => {
                        tracing::error!("error: {:?}", e);
                        Err("error".to_string())
                    }
                }
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
