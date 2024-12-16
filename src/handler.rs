use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use command::command::{
    create_circle::{self, Input, Output},
    update_circle,
};
use serde::Deserialize;
use std::env;

pub async fn handle_get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

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

// #[derive(Debug, Deserialize)]
// pub struct FetchCircleInputParam {
//     id: String,
// }

// #[derive(Debug, serde::Deserialize, serde::Serialize)]
// pub struct FetcheCircleResponseBody {
//     pub circle_id: String,
//     pub circle_name: String,
//     pub capacity: i16,
//     pub owner: MemberOutput,
//     pub members: Vec<MemberOutput>,
// }

// impl std::convert::From<FetchCircleOutput> for FetcheCircleResponseBody {
//     fn from(
//         FetchCircleOutput {
//             circle_id,
//             circle_name,
//             capacity,
//             owner,
//             members,
//         }: FetchCircleOutput,
//     ) -> Self {
//         FetcheCircleResponseBody {
//             circle_id,
//             circle_name,
//             capacity,
//             owner,
//             members,
//         }
//     }
// }

// pub async fn handle_fetch_circle(
//     State(state): State<AppState>,
//     Path(param): Path<FetchCircleInputParam>,
// ) -> Result<Json<FetcheCircleResponseBody>, String> {
//     let fetch_circle_input = FetchCircleInput::new(param.id);
//     let usecase = FetchCircleUsecase::new(state.circle_repository);
//     usecase
//         .execute(fetch_circle_input)
//         .await
//         .map(FetcheCircleResponseBody::from)
//         .map(Json)
//         .map_err(|e| e.to_string())
// }

#[derive(Debug, Deserialize)]
pub struct UpdateCircleInputParam {
    id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UpdateCircleRequestBody {
    pub circle_name: Option<String>,
    pub capacity: Option<i16>,
}

impl UpdateCircleRequestBody {
    pub fn into_to_input(self, id: String) -> update_circle::Input {
        update_circle::Input {
            circle_id: id,
            circle_name: self.circle_name,
            capacity: self.capacity,
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
                update_circle::Error::Duplicate => Err(StatusCode::CONFLICT),
                update_circle::Error::Circle => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}
