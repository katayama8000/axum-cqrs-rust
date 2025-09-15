use std::sync::Arc;

use api::{app_state::AppState, router::router};

use crate::{
    config::{connect::connect as mysql_connect, redis_connect::connect as redis_connect},
    injectors::{
        build_command_handler::build_command_handler, build_query_handler::build_query_handler,
    },
};

pub async fn run() -> Result<(), ()> {
    tracing_subscriber::fmt().init();

    let mysql_pool = mysql_connect().await.expect("MySQL should connect");
    let redis_client = redis_connect().expect("Redis should connect");

    let command_handler = build_command_handler(mysql_pool);
    let query_handler = build_query_handler(redis_client);
    let state = AppState::new(Arc::new(command_handler), Arc::new(query_handler));

    let app = router().with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("server should bind to port");
    println!(
        "Listening on: {}",
        listener.local_addr().expect("server should bind to port")
    );
    axum::serve(listener, app).await.expect("server should run");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::config::{connect::connect_test, redis_connect::connect_test as redis_connect_test};
    use api::{
        app_state::AppState,
        handler::{CreateCircleRequestBody, CreateCircleResponseBody, UpdateCircleRequestBody},
        router::router,
    };
    use axum::{
        http::{header::CONTENT_TYPE, StatusCode},
        Router,
    };
    use domain::aggregate::value_object::circle_id::CircleId;
    use tower::ServiceExt;

    use super::*;

    // FIXME: ignore test because it requires a running database
    #[tokio::test]
    #[ignore]
    async fn test_version() -> anyhow::Result<()> {
        let mysql_pool = connect_test().await.expect("database should connect");
        let redis_client = redis_connect_test().expect("Redis should connect");
        let command_handler = build_command_handler(mysql_pool);
        let query_handler = build_query_handler(redis_client);
        let state = AppState::new(Arc::new(command_handler), Arc::new(query_handler));
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

    // #[tokio::test]
    // #[ignore]
    // async fn test_create_circle() -> anyhow::Result<()> {
    //     let pool = connect_test().await.expect("database should connect");
    //     let command_handler = build_command_handler(pool.clone());
    //     let query_handler = build_query_handler(pool.clone());
    //     let state = AppState::new(Arc::new(command_handler), Arc::new(query_handler));
    //     let app = router().with_state(state.clone());
    //     let response = app
    //         .oneshot(
    //             axum::http::Request::builder()
    //                 .method("POST")
    //                 .uri("/circle")
    //                 .header(CONTENT_TYPE, "application/json")
    //                 .body(axum::body::Body::new(serde_json::to_string(
    //                     &CreateCircleRequestBody {
    //                         circle_name: "circle_name1".to_string(),
    //                         capacity: 10,
    //                         owner_name: "owner1".to_string(),
    //                         owner_age: 21,
    //                         owner_grade: 3,
    //                         owner_major: "Music".to_string(),
    //                     },
    //                 )?))?,
    //         )
    //         .await?;
    //     assert_eq!(response.status(), StatusCode::OK);
    //     let response_body = serde_json::from_slice::<'_, CreateCircleResponseBody>(
    //         &axum::body::to_bytes(response.into_body(), usize::MAX).await?,
    //     )?;

    //     let created = state
    //         .command_handler()
    //         .circle_repository()
    //         .find_by_id(&CircleId::from_str(&response_body.circle_id)?)
    //         .await?;
    //     let circle = Circle::reconstruct(
    //         CircleId::from_str(&response_body.circle_id)?,
    //         "circle_name1".to_string(),
    //         Member::reconstruct(
    //             MemberId::from_str(&response_body.owner_id)?,
    //             "owner1".to_string(),
    //             21,
    //             Grade::try_from(3)?,
    //             Major::Music,
    //             Version::new(),
    //         ),
    //         10,
    //         vec![],
    //         Version::new(),
    //     );
    //     assert_eq!(created, circle);
    //     Ok(())
    // }

    #[tokio::test]
    #[ignore]
    async fn test_fetch_circle() -> anyhow::Result<()> {
        let mysql_pool = connect_test().await.expect("database should connect");
        let redis_client = redis_connect_test().expect("Redis should connect");
        let command_handler = build_command_handler(mysql_pool);
        let query_handler = build_query_handler(redis_client);
        let state = AppState::new(Arc::new(command_handler), Arc::new(query_handler));
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

        let circle_id = build_circle(&app).await?;

        let fetched_response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri(format!("/circle/{}", circle_id))
                    .body(axum::body::Body::empty())?,
            )
            .await?;
        assert_eq!(fetched_response.status(), StatusCode::OK);
        let _fetched_response_body = String::from_utf8(
            axum::body::to_bytes(fetched_response.into_body(), usize::MAX)
                .await?
                .to_vec(),
        )?;

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_update_circle() -> anyhow::Result<()> {
        let mysql_pool = connect_test().await.expect("database should connect");
        let redis_client = redis_connect_test().expect("Redis should connect");
        let command_handler = build_command_handler(mysql_pool);
        let query_handler = build_query_handler(redis_client);
        let state = AppState::new(Arc::new(command_handler), Arc::new(query_handler));
        let app = router().with_state(state.clone());
        let circle_id = build_circle(&app).await?;
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
                            version: 1,
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(update_response.status(), StatusCode::OK);

        let updated_circle = state
            .command_handler
            .circle_repository()
            .find_by_id(&CircleId::from_str(&circle_id)?)
            .await?;
        assert_eq!(updated_circle.name, "Football club");
        assert_eq!(updated_circle.capacity, 20);

        Ok(())
    }

    async fn build_circle(app: &Router) -> anyhow::Result<String> {
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
                        },
                    )?))?,
            )
            .await?;
        assert_eq!(create_response.status(), StatusCode::OK);
        let create_response_body = serde_json::from_slice::<CreateCircleResponseBody>(
            &axum::body::to_bytes(create_response.into_body(), usize::MAX).await?,
        )?;

        Ok(create_response_body.circle_id)
    }
}