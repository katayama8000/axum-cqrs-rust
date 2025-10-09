use std::sync::Arc;

use infrastructure::{
    circle_duplicate_checker::CircleDuplicateChecker, 
    circle_repository::CircleRepository,
    event_publisher::{EventPublisher, InMemoryEventPublisher, RedisProjectionHandler},
};

use super::command_handler_impl::CommandHandlerImpl;

pub fn build_command_handler(
    db: sqlx::MySqlPool,
    redis_client: redis::Client,
) -> CommandHandlerImpl {
    let (event_publisher, event_receiver) = InMemoryEventPublisher::new();
    let event_publisher: Arc<dyn EventPublisher> = Arc::new(event_publisher);
    
    let redis_handler = RedisProjectionHandler::new(redis_client.clone(), db.clone());
    
    tokio::spawn(async move {
        redis_handler.start_processing(event_receiver).await;
    });

    let circle_repository = Arc::new(CircleRepository::new(
        db.clone(), 
        event_publisher
    ));
    let circle_duplicate_checker = Arc::new(CircleDuplicateChecker::new(db.clone()));

    CommandHandlerImpl {
        circle_repository,
        circle_duplicate_checker,
    }
}
