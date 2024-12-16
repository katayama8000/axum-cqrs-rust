use std::sync::Arc;

use crate::{
    config::connect::connect,
    handler::{handle_create_circle, handle_fetch_all, handle_fetch_circle, handle_update_circle},
};

use axum::{
    routing::{get, post, put},
    Router,
};
use command::command_handler::{CommandHandler, HasCommandHandler};
use handler::{handle_debug, handle_get_version};
use infrastructure::{
    circle_duplicate_checker::CircleDuplicateCheckerWithMySql,
    circle_repository_with_my_sql::CircleRepositoryWithMySql,
};
use injectors::build_command_handler::build_command_handler;

mod config;
mod handler;
mod injectors;

#[derive(Clone)]
struct AppState {
    circle_repository: CircleRepositoryWithMySql,
    circle_duplicate_checker: CircleDuplicateCheckerWithMySql,
    command_handler: Arc<dyn CommandHandler + Send + Sync>,
}

impl AppState {
    pub fn new(
        circle_repository: CircleRepositoryWithMySql,
        circle_duplicate_checker: CircleDuplicateCheckerWithMySql,
        command_handler: Arc<dyn CommandHandler + Send + Sync>,
    ) -> Self {
        Self {
            circle_repository,
            circle_duplicate_checker,
            command_handler,
        }
    }
}

impl HasCommandHandler for AppState {
    fn command_handler(&self) -> Arc<dyn CommandHandler + Send + Sync> {
        self.command_handler.clone()
    }
}

fn router() -> Router<AppState> {
    Router::new()
        .route("/version", get(handle_get_version))
        .route("/circle/:id", get(handle_fetch_circle))
        .route("/circle", get(handle_fetch_all))
        .route("/circle", post(handle_create_circle))
        .route("/circle/:id", put(handle_update_circle))
        .route("/debug", get(handle_debug))
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().init();

    let pool = connect().await.expect("database should connect");

    let command_handler = build_command_handler(pool.clone());
    let state = AppState::new(
        CircleRepositoryWithMySql::new(pool.clone()),
        CircleDuplicateCheckerWithMySql::new(pool.clone()),
        Arc::new(command_handler),
    );

    let app = router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        config::connect::connect_test,
        handler::{CreateCircleRequestBody, CreateCircleResponseBody, UpdateCircleRequestBody},
    };
    use axum::http::{header::CONTENT_TYPE, StatusCode};
    use domain::{
        aggregate::{
            circle::Circle,
            member::Member,
            value_object::{circle_id::CircleId, grade::Grade, major::Major, member_id::MemberId},
        },
        interface::circle_repository_interface::CircleRepositoryInterface,
    };
    use tower::ServiceExt;

    use super::*;

    // FIXME: ignore test because it requires a running database
    #[tokio::test]
    #[ignore]
    async fn test_version() -> anyhow::Result<()> {
        let pool = connect_test().await.expect("database should connect");
        let command_handler = build_command_handler(pool.clone());
        let state = AppState {
            circle_repository: CircleRepositoryWithMySql::new(pool.clone()),
            circle_duplicate_checker: CircleDuplicateCheckerWithMySql::new(pool.clone()),
            command_handler: Arc::new(command_handler),
        };
        let app = router().with_state(state);
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/version")
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(response_body, "0.1.0-rc.1");
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_circle() -> anyhow::Result<()> {
        let pool = connect_test().await.expect("database should connect");
        let command_handler = build_command_handler(pool.clone());
        let state = AppState {
            circle_repository: CircleRepositoryWithMySql::new(pool.clone()),
            circle_duplicate_checker: CircleDuplicateCheckerWithMySql::new(pool.clone()),
            command_handler: Arc::new(command_handler),
        };
        let app = router().with_state(state.clone());
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/circle")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &CreateCircleRequestBody {
                            circle_name: "circle_name1".to_string(),
                            capacity: 10,
                            owner_name: "owner1".to_string(),
                            owner_age: 21,
                            owner_grade: 3,
                            owner_major: "Music".to_string(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = serde_json::from_slice::<'_, CreateCircleResponseBody>(
            &axum::body::to_bytes(response.into_body(), usize::MAX).await?,
        )?;

        let created = state
            .circle_repository
            .find_by_id(&CircleId::from_str(&response_body.circle_id)?)
            .await?;
        let circle = Circle::reconstruct(
            CircleId::from_str(&response_body.circle_id)?,
            "circle_name1".to_string(),
            Member::reconstruct(
                MemberId::from_str(&response_body.owner_id)?,
                "owner1".to_string(),
                21,
                Grade::try_from(3)?,
                Major::Music,
            ),
            10,
            vec![],
        );
        assert_eq!(created, circle);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_fetch_circle() -> anyhow::Result<()> {
        let pool = connect_test().await.expect("database should connect");
        let command_handler = build_command_handler(pool.clone());
        let state = AppState {
            circle_repository: CircleRepositoryWithMySql::new(pool.clone()),
            circle_duplicate_checker: CircleDuplicateCheckerWithMySql::new(pool.clone()),
            command_handler: Arc::new(command_handler),
        };
        let app = router().with_state(state);
        let unexist_circle_id = 0;
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri(format!("/circle/{}", unexist_circle_id))
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        let response_body = String::from_utf8(
            axum::body::to_bytes(response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(response_body, "Circle not found");

        let (circle_id, owner_id) = build_circle(&app).await?;

        let fetched_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri(format!("/circle/{}", circle_id))
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(fetched_response.status(), StatusCode::OK);
        let fetched_response_body = String::from_utf8(
            axum::body::to_bytes(fetched_response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;
        assert_eq!(
            fetched_response_body,
            format!(
                "{{\"circle_id\":{},\"circle_name\":\"Music club\",\"capacity\":10,\"owner\":{{\"id\":{},\"name\":\"John Lennon\",\"age\":21,\"grade\":3,\"major\":\"Music\"}},\"members\":[]}}",
                circle_id,owner_id
            )
        );
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_circle() -> anyhow::Result<()> {
        let pool = connect_test().await.expect("database should connect");
        let command_handler = build_command_handler(pool.clone());
        let state = AppState {
            circle_repository: CircleRepositoryWithMySql::new(pool.clone()),
            circle_duplicate_checker: CircleDuplicateCheckerWithMySql::new(pool.clone()),
            command_handler: Arc::new(command_handler),
        };
        let app = router().with_state(state.clone());
        let (circle_id, _) = build_circle(&app).await?;
        let update_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("PUT")
                    .uri(format!("/circle/{}", circle_id))
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &UpdateCircleRequestBody {
                            circle_name: Some("Football club".to_string()),
                            capacity: Some(20),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(update_response.status(), StatusCode::OK);

        let updated_circle = state
            .circle_repository
            .find_by_id(&CircleId::from_str(&circle_id)?)
            .await?;
        assert_eq!(updated_circle.name, "Football club");
        assert_eq!(updated_circle.capacity, 20);

        Ok(())
    }

    async fn build_circle(app: &Router) -> anyhow::Result<(String, String)> {
        let create_response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/circle")
                    .header(CONTENT_TYPE, "application/json")
                    .body(axum::body::Body::new(serde_json::to_string(
                        &CreateCircleRequestBody {
                            circle_name: "Music club".to_string(),
                            capacity: 10,
                            owner_name: "John Lennon".to_string(),
                            owner_age: 21,
                            owner_grade: 3,
                            owner_major: "Music".to_string(),
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(create_response.status(), StatusCode::OK);
        let create_response_body = serde_json::from_slice::<CreateCircleResponseBody>(
            &axum::body::to_bytes(create_response.into_body(), usize::MAX).await?,
        )?;

        Ok((
            create_response_body.circle_id,
            create_response_body.owner_id,
        ))
    }
}
