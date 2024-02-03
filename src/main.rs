mod prompt;
mod error;
use chrono::prelude::NaiveDate;
use std::io::{self, Write};
use rustic_boards::{TaskItem, TimeStamp};
use prompt::{text_input_prompt, confirm_prompt, date_input_prompt};

fn main() {
    loop {
        let mut user_input: String = String::new();
        print!("boards> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        let input_parts: Vec<&str> = user_input.trim()
            .split_whitespace()
            .collect::<Vec<&str>>();

        match input_parts.as_slice() {
            ["add", "task"] => {
                let task_name: String = text_input_prompt("Task Name:").unwrap();
                let task_description: String = text_input_prompt("Task Description:").unwrap();
                let deadline_check: bool = confirm_prompt(
                    "Is there a deadline for this task?", 
                    Some("It's recommended to set a deadline to track for completion.")
                ).unwrap();
                
                let task_deadline: Option<TimeStamp> = match deadline_check {
                    true => {
                        let input_deadline: NaiveDate = date_input_prompt("Task Deadline:").unwrap();                        
                        Some(TimeStamp::convert(input_deadline))
                    },
                    false => None
                };
                
                TaskItem::new(task_name, task_description, task_deadline).unwrap();
            },
            ["get", key] => {
                println!("No secret found for key: {}", key);
            },
            ["exit"] => {
                break;
            },
            _ => {
                println!("Invalid command!");
            }
        }
    }
}
