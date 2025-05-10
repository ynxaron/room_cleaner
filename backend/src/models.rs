use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Room {
    pub room_no: u32,
    pub is_occupied: bool,
    pub is_clean: bool,
}

#[derive(Serialize)]
pub struct Task {
    pub room_no: u32,
    pub priority: u32,
}
