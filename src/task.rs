
#[derive(Debug)]
pub(crate) struct Task {
    pub(crate) description: String,
    pub(crate) completed: bool,
}

impl Task {
    pub(crate) fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }
}
