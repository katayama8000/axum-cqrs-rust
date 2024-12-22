use std::sync::Arc;

use domain::{
    aggregate::circle::Circle, interface::query::circle_reader_interface::CircleReaderInterface,
};

pub struct Input;

pub struct Output(pub Vec<Circle>);

pub async fn handle(
    circle_reader: Arc<dyn CircleReaderInterface + Send + Sync>,
    Input {}: Input,
) -> Result<Output, anyhow::Error> {
    let circles = circle_reader
        .list_circles()
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(Output(circles))
}
