mod circle_created;
mod circle_deleted;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CircleEvent {
    CircleCreated(circle_created::CircleCreated),
    CircleDeleted(circle_deleted::CircleDeleted),
}
