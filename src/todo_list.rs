use crate::task::Task;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use chrono::NaiveDate;

pub(crate) struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub(crate) fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
        }
    }

    pub(crate) fn add_task(&mut self, description: String, priority: u8, deadline: Option<NaiveDate>) {
        let task = Task::new(description, priority, deadline);
        self.tasks.push(task);
        self.tasks.sort_by_key(|t| t.priority); // Keep tasks sorted by priority
    }

    pub(crate) fn show_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { " " };
            let deadline_str = if let Some(deadline) = task.deadline {
                format!(" (Deadline: {})", deadline)
            } else {
                String::from("")
            };
            println!("{}: [{}] {} (Priority: {}){}", index + 1, status, task.description, task.priority, deadline_str);
        }
    }

    pub(crate) fn complete_task(&mut self, index: usize) -> Result<(), String> {
        if index < 1 || index > self.tasks.len() {
            return Err("Invalid task number.".to_string());
        }
        self.tasks[index - 1].completed = true;
        Ok(())
    }

    pub(crate) fn save_to_file(&self, filename: &str) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        for task in &self.tasks {
            writeln!(file, "{},{},{},{:?}", task.description, task.completed, task.priority, task.deadline)?;
        }

        Ok(())
    }

    pub(crate) fn load_from_file(filename: &str) -> Result<TodoList, io::Error> {
        let mut todo_list = TodoList::new();
        let file = OpenOptions::new()
            .read(true)
            .open(filename);

        if let Ok(file) = file {
            use std::io::BufReader;
            use std::io::BufRead;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 4 {
                    let description = parts[0].to_string();
                    let completed = parts[1].parse().unwrap_or(false);
                    let priority = parts[2].parse().unwrap_or(5);
                    let deadline = if priority <= 5 { Some(NaiveDate::parse_from_str(parts[3], "%Y-%m-%d").unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())) } else { None };
                    todo_list.tasks.push(Task { description, completed, priority, deadline });
                }
            }
        }

        Ok(todo_list)
    }

    pub(crate) fn edit_task(&mut self, index: usize, new_description: String) -> Result<(), String> {
        if index < 1 || index > self.tasks.len() {
            return Err("Invalid task number.".to_string());
        }
        self.tasks[index - 1].description = new_description;
        Ok(())
    }

    pub(crate) fn delete_task(&mut self, index: usize) -> Result<(), String> {
        if index < 1 || index > self.tasks.len() {
            return Err("Invalid task number.".to_string());
        }
        self.tasks.remove(index - 1);
        Ok(())
    }
}
