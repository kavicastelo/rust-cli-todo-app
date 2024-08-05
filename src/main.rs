use std::io::{self, Write};

mod task;
mod todo_list;

use todo_list::TodoList;

fn main() {
    println!("Welcome to the To-Do List CLI Application!");

    let filename = "todo_list.txt";
    let mut todo_list = TodoList::load_from_file(filename).unwrap_or_else(|_| {
        println!("Creating a new to-do list.");
        TodoList::new()
    });

    loop {
        println!("\nWhat would you like to do?");
        println!("1. Add a task");
        println!("2. View tasks");
        println!("3. Complete a task");
        println!("4. Save and exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todo_list.add_task(description.trim().to_string());
            },
            "2" => {
                todo_list.show_tasks();
            },
            "3" => {
                print!("Enter task number to complete: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();

                match index_str.trim().parse::<usize>() {
                    Ok(index) => {
                        if let Err(e) = todo_list.complete_task(index) {
                            println!("{}", e);
                        }
                    },
                    Err(_) => println!("Please enter a valid number."),
                }
            },
            "4" => {
                if let Err(e) = todo_list.save_to_file(filename) {
                    println!("Failed to save to-do list: {}", e);
                }
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}
