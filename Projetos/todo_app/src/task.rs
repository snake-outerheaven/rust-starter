use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub created_at: DateTime<Local>,
    pub finished_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(
        id: i32,
        name: String,
        desc: String,
        created_at: DateTime<Local>,
        finished_at: Option<DateTime<Local>>,
    ) -> Self {
        Self {
            id: (id),
            name: (name),
            desc: (desc),
            created_at: (created_at),
            finished_at: (finished_at),
        }
    }
}
