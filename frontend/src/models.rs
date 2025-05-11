use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Room {
    pub room_no: u32,
    pub is_occupied: bool,
    pub is_clean: bool,
    pub urgency: u8,
    pub cleaner_speed: u8,
    pub booked_long: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Task {
    pub room_no: u32,
    pub priority: u32,
}
