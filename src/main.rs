use std::io::{self, Write};
use chrono::NaiveDate;
use clap::{Arg, Command};

mod task;
mod todo_list;

use todo_list::TodoList;

fn main() {
    println!("Welcome to the To-Do List CLI Application!");

    let matches = Command::new("Todo CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A simple to-do list CLI application")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE")
            .help("Specifies the to-do list file")
            .default_value("todo_list.txt"))
        .get_matches();

    let filename = matches.get_one::<String>("file").map(|s| s.as_str()).unwrap_or("todo_list.txt");

    let mut todo_list = TodoList::load_from_file(filename).unwrap_or_else(|_| {
        println!("Creating a new to-do list.");
        TodoList::new()
    });

    loop {
        println!("\nWhat would you like to do?");
        println!("1. Add a task");
        println!("2. View tasks");
        println!("3. Complete a task");
        println!("4. Edit a task");
        println!("5. Save and exit");
        println!("6. Exit without saving");

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

                print!("Enter task priority (1-5, 1 is highest): ");
                io::stdout().flush().unwrap();
                let mut priority_str = String::new();
                io::stdin().read_line(&mut priority_str).unwrap();

                let priority = priority_str.trim().parse::<u8>().unwrap_or(5);

                print!("Enter task deadline (YYYY-MM-DD) or press Enter to skip: ");
                io::stdout().flush().unwrap();
                let mut deadline_str = String::new();
                io::stdin().read_line(&mut deadline_str).unwrap();

                let deadline = if deadline_str.trim().is_empty() {
                    None
                } else {
                    Some(NaiveDate::parse_from_str(deadline_str.trim(), "%Y-%m-%d").unwrap_or_else(|_| {
                        println!("Invalid date format, skipping deadline.");
                        NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
                    }))
                };

                todo_list.add_task(description.trim().to_string(), priority, deadline);
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
                print!("Enter task number to edit: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();

                match index_str.trim().parse::<usize>() {
                    Ok(index) => {
                        print!("Enter new task description: ");
                        io::stdout().flush().unwrap();
                        let mut new_description = String::new();
                        io::stdin().read_line(&mut new_description).unwrap();

                        if let Err(e) = todo_list.edit_task(index, new_description.trim().to_string()) {
                            println!("{}", e);
                        }
                    },
                    Err(_) => println!("Please enter a valid number."),
                }
            },
            "5" => {
                if let Err(e) = todo_list.save_to_file(filename) {
                    println!("Failed to save to-do list: {}", e);
                }
                println!("Goodbye!");
                break;
            },
            "6" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}
