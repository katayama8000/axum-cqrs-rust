pub fn is_valid(capacity: i16) -> bool {
    capacity >= 3
}

pub fn apply_update(current: i16, proposed: Option<i16>) -> i16 {
    match proposed {
        Some(capacity) => capacity,
        None => current,
    }
}
