use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Room {
    pub room_no: u32,
    pub is_occupied: bool,
    pub is_clean: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Task {
    pub room_no: u32,
    pub priority: u32,
}
