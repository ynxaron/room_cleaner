use crate::models::{Room, Task};

pub fn schedule_tasks(rooms: Vec<Room>) -> Vec<Task> {
    let mut tasks: Vec<Task> = rooms
        .into_iter()
        .filter(|r| !r.is_clean)
        .map(|r| {
            let mut priority = 0;

            // Dirty room
            priority += 1;

            // Unoccupied gets higher priority
            if !r.is_occupied {
                priority += 2;
            }

            // Urgency factor: scaled 0-10
            priority += r.urgency as u32;

            // Cleaner speed factor: scaled 0-10
            priority += r.cleaner_speed as u32;

            // Booked long gets a bit of weight
            if r.booked_long {
                priority += 1;
            }

            Task {
                room_no: r.room_no,
                priority,
            }
        })
        .collect();

    tasks.sort_by_key(|t| std::cmp::Reverse(t.priority));
    tasks
}
