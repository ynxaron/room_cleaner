use crate::models::{Room, Task};

pub fn schedule_tasks(rooms: Vec<Room>) -> Vec<Task> {
    let mut tasks: Vec<Task> = rooms
        .into_iter()
        .filter(|r| !r.is_clean)
        .map(|r| {
            let mut priority = 0;
            if !r.is_occupied {
                priority += 2;
            }
            priority += 1; // dirty always increases priority
            Task {
                room_no: r.room_no,
                priority,
            }
        })
        .collect();

    tasks.sort_by_key(|t| std::cmp::Reverse(t.priority));
    tasks
}
