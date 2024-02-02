use chrono::prelude::*;
use inquire::{Text, formatter::DEFAULT_DATE_FORMATTER, CustomType, Confirm, ui::RenderConfig, validator::Validation};
use std::io::{self, Write};
use rustic_boards::{TaskItem, TimeStamp};

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
                let task_name: String = Text::new("Task Name:").prompt().unwrap();
                let task_description: String = Text::new("Task Description:").prompt().unwrap();
                let deadline_check: bool = Confirm{
                        message: "Is there a deadline for this task?",
                        default: Some(false),
                        placeholder: Some("yes|no"),
                        help_message: Some("It's recommended to set a deadline to track for completion."),
                        formatter: &|ans| match ans {
                            true => "yes".to_owned(),
                            false => "no".to_owned(),
                        },
                        parser: &|ans| match ans {
                            "yes" => Ok(true),
                            "no" => Ok(false),
                            _ => Err(()),
                        },
                        error_message: "Reply with 'yes' or 'no'".into(),
                        default_value_formatter: &|def| match def {
                            true => String::from("yes"),
                            false => String::from("no"),
                        },
                        render_config: RenderConfig::default(),
                    }
                    .prompt()
                    .unwrap();
                
                let task_deadline: Option<TimeStamp> = match deadline_check {
                    true => {
                        let input_deadline: NaiveDate = CustomType::<NaiveDate>::new("Task Deadline:")
                            .with_placeholder("dd/mm/yyyy")
                            .with_parser(&|i| NaiveDate::parse_from_str(i, "%d/%m/%Y").map_err(|_e| ()))
                            .with_formatter(DEFAULT_DATE_FORMATTER)
                            .with_error_message("Please enter a valid date in dd/mm/yyyy format.")
                            .with_validator(|val: &NaiveDate| {
                                if val < &Local::now().date_naive().into() {
                                    Ok(Validation::Invalid(
                                        "Task deadline cannot be prior to current date.".into(),
                                    ))
                                } else {
                                    Ok(Validation::Valid)
                                }
                            })
                            .prompt()
                            .unwrap();

                        Some(TimeStamp::convert(input_deadline))
                    },
                    false => None
                };
                
                let task: TaskItem = TaskItem::new(task_name, task_description, task_deadline);
                println!("{:#?}", task);
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
