use chrono::{NaiveDate};

#[derive(Debug)]
pub(crate) struct Task {
    pub(crate) description: String,
    pub(crate) completed: bool,
    pub(crate) priority: u8, // Priority from 1 (high) to 5 (low)
    pub(crate) deadline: Option<NaiveDate>,
}

impl Task {
    pub(crate) fn new(description: String, priority: u8, deadline: Option<NaiveDate>) -> Task {
        Task {
            description,
            completed: false,
            priority,
            deadline,
        }
    }
}
