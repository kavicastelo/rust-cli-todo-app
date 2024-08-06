# Rust To-Do List CLI Application

A simple and efficient command-line application to manage a to-do list, built using Rust. This application supports features like adding tasks, viewing tasks, marking tasks as complete, editing tasks, deleting tasks, prioritizing tasks, and setting deadlines.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Example Workflow](#example-workflow)
- [Dependencies](#dependencies)
- [Contributing](#contributing)
- [License](#license)

## Features
- **Add Tasks:** Create new tasks with descriptions, priorities, and optional deadlines.
- **View Tasks:** Display all current tasks with their statuses, priorities, and deadlines.
- **Complete Tasks:** Mark tasks as complete once they're done.
- **Edit Tasks:** Modify existing tasks to update their descriptions, priorities, or deadlines.
- **Delete Tasks:** Remove tasks that are no longer needed.
- **Prioritize Tasks:** Assign priorities to tasks to keep track of their importance.
- **Set Deadlines:** Assign due dates to tasks to manage your time effectively.
- **Color-Coded Menu:** The application menu is color-coded for better visual distinction.

## Installation
To use this application, you'll need to have Rust installed on your system. You can install Rust from the [official website](https://www.rust-lang.org/tools/install).

### Step-by-Step Instructions:
1. Clone the Repository:
    ```bash
    git clone https://github.com/kavicastelo/rust_todo_cli.git
    cd rust_todo_cli
    ```
   
2. Build the Application:
    ```bash
    cargo build --release
    ```
   
3. Run the Application:
    ```bash
    cargo run
    ```
   
## Usage
Once the application is running, you can interact with it via the command-line interface. Follow the prompts to manage your to-do list.

## Commands
Here’s a list of available commands:

1. Add a Task:
   - Enter a task description, priority (1-5), and optional deadline (YYYY-MM-DD).
2. View Tasks:
   - Displays all tasks, including their statuses, priorities, and deadlines.
3. Complete a Task:
   - Mark a task as complete by entering its number.
4. Edit a Task:
   - Modify an existing task's description, priority, or deadline.
5. Delete a Task:
   - Remove a task from the list by entering its number.
6. Exit:
   - Save the current state of the to-do list and exit the application.

## Example Workflow
1. Add a New Task:
    ```bash
   1. Add a task
    Enter task description: Finish writing the report
    Enter task priority (1-5, 1 is highest): 2
    Enter task deadline (YYYY-MM-DD) or press Enter to skip: 2024-08-10
    ```

2. View All Tasks:
    ```bash
   2. View tasks
    1: [ ] Finish writing the report (Priority: 2) (Deadline: 2024-08-10)
    ```
   
3. Complete a Task:
    ```bash
   3. Complete a task
    Enter task number to complete: 1
    ```
   
4. Edit a Task:
    ```bash
   4. Edit a task
    Enter task number to edit: 1
    Enter task description: Finish writing the report
    ```
   
5. Delete a Task:
    ```bash
   5. Delete a task
    Enter task number to delete: 1
    ```
   
6. Exit:
    ```bash
   6. Exit
    ```
   
## Dependencies
The application relies on the following Rust crates:
- `crossterm`: Used for terminal manipulation (clearing the screen, adding colors).
- `chrono`: Used for handling date and time, specifically for task deadlines.
- `serde`: (optional, for future enhancements like saving/loading tasks from files).
- `clap`: For command-line argument parsing (future enhancement).

## Contributing
Contributions are welcome! If you have suggestions or improvements, feel free to submit a pull request.

### How to Contribute:
1. Fork the repository.
2. Create a new branch: ``git checkout -b my-feature-branch.
3. Make your changes and commit them: `git commit -m 'Add some feature`'.
4. Push to the branch: `git push origin my-feature-branch`.
5. Submit a pull request.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.