mod prompt;
mod error;
use error::AppError;
use chrono::prelude::NaiveDate;
use std::io::{self, Write};
use rustic_boards::{TaskStatus, TaskItem, TaskPriority, TimeStamp, KanbanBoard};
use prompt::{text_input_prompt, confirm_prompt, date_input_prompt, select_prompt};

fn main() {
    let boards_file_exists: bool = KanbanBoard::check_if_file_exists().unwrap();
    let mut boards: KanbanBoard = match boards_file_exists {
        true => {
            KanbanBoard::load_from_file().unwrap()
        },
        false => {
            KanbanBoard::new()
        }
    };

    if boards_file_exists == false {
        boards.add_to_board(String::new(), TaskStatus::ToDo);
        boards.add_to_board(String::new(), TaskStatus::InProgress);
        boards.add_to_board(String::new(), TaskStatus::Blocked);
        boards.add_to_board(String::new(), TaskStatus::InReview);
        boards.add_to_board(String::new(), TaskStatus::Done);
        boards.write_to_file().unwrap();
    };

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
                let task_priority: TaskPriority = select_prompt("Task Priority:").unwrap();
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
                
                let task_item: TaskItem = TaskItem::new(task_name, task_description, task_deadline, task_priority).unwrap();
                task_item.write_to_file().unwrap();
                boards.add_to_board(task_item.task_id, task_item.task_status);
            },
            ["open", "task", task_id] => {
                match TaskItem::check_if_file_exists(&task_id.to_string()).unwrap() {
                    true => {
                        let task_item: TaskItem = TaskItem::get_task(&task_id.to_string()).unwrap();
                        println!("{:#?}\n", task_item);
                    },
                    false => println!("{}\n", AppError::TaskNotFound(task_id.to_string()))
                }
            },
            ["show", swimlane] => {
                match boards.show(swimlane) {
                    Ok(_) => {},
                    Err(e) => println!("{}", e)
                };
            },
            ["exit"] => {
                break;
            },
            _ => {
                println!("{}", AppError::InvalidCommand(user_input));
            }
        }
    }
}
