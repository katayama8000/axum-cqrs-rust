use std::sync::Arc;

use crate::interface::list_circles_reader_interface::ListCirclesReaderInterface;
use domain::aggregate::circle::Circle;

pub struct Input;

pub struct Output(pub Vec<Circle>);

pub async fn handle(
    list_circles_reader: Arc<dyn ListCirclesReaderInterface + Send + Sync>,
    Input {}: Input,
) -> Result<Output, anyhow::Error> {
    let circles = list_circles_reader
        .list_circles()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(circles)
}
