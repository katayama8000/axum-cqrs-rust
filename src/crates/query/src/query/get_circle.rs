use std::{str::FromStr, sync::Arc};

use anyhow::Ok;
use domain::aggregate::{circle::Circle, value_object::circle_id::CircleId};

use crate::interface::circle_reader_interface::CircleReaderInterface;

pub struct Input {
    pub circle_id: String,
}

pub struct Output(pub Option<Circle>);

pub async fn handle(
    circle_reader: Arc<dyn CircleReaderInterface + Send + Sync>,
    Input { circle_id }: Input,
) -> Result<Output, anyhow::Error> {
    let circle_id = CircleId::from_str(circle_id.as_str())?;
    let circle = circle_reader
        .get_circle(circle_id)
        .await
        .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    Ok(Output(circle))
}
