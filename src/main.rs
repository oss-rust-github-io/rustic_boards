mod boards;
mod constants;
mod error;
mod links;
mod prompt;
mod subtasks;
mod tasks;
mod utils;

use boards::KanbanBoard;
use chrono::prelude::NaiveDate;
use error::AppError;
use links::TaskToSubtaskMap;
use prompt::{
    confirm_prompt, date_input_prompt, select_prompt, tasks_select_prompt, text_input_prompt,
};
use std::io::{self, Write};
use subtasks::SubTaskItem;
use tasks::TaskItem;
use utils::{TaskPriority, TaskStatus, TimeStamp};

fn main() {
    let boards_file_exists: bool = KanbanBoard::check_if_file_exists().unwrap();
    let mut boards: KanbanBoard = match boards_file_exists {
        true => KanbanBoard::load_from_file().unwrap(),
        false => KanbanBoard::new(),
    };

    if boards_file_exists == false {
        boards.add_to_board(String::new(), TaskStatus::ToDo).unwrap_or_else(|err| {
            println!("{}", err.to_string());
        });
        boards.add_to_board(String::new(), TaskStatus::InProgress).unwrap_or_else(|err| {
            println!("{}", err.to_string());
        });
        boards.add_to_board(String::new(), TaskStatus::Blocked).unwrap_or_else(|err| {
            println!("{}", err.to_string());
        });
        boards.add_to_board(String::new(), TaskStatus::InReview).unwrap_or_else(|err| {
            println!("{}", err.to_string());
        });
        boards.add_to_board(String::new(), TaskStatus::Done).unwrap_or_else(|err| {
            println!("{}", err.to_string());
        });
        boards.write_to_file().unwrap_or_else(|err| {
            println!("{}", err.to_string());
        });
    };

    let links_file_exists: bool = TaskToSubtaskMap::check_if_file_exists().unwrap();
    let mut tasks_link: TaskToSubtaskMap = match links_file_exists {
        true => TaskToSubtaskMap::load_from_file().unwrap(),
        false => TaskToSubtaskMap::new(),
    };

    loop {
        let mut user_input: String = String::new();
        print!("boards> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        let input_parts: Vec<&str> = user_input.trim().split_whitespace().collect::<Vec<&str>>();

        match input_parts.as_slice() {
            ["add", "task"] => {
                let task_name: String = text_input_prompt("Task Name:", None).unwrap();
                let task_description: String =
                    text_input_prompt("Task Description:", None).unwrap();
                let task_priority: TaskPriority = select_prompt("Task Priority:").unwrap();
                let deadline_check: bool = confirm_prompt(
                    "Is there a deadline for this task?",
                    Some("It's recommended to set a deadline to track for completion."),
                )
                .unwrap();

                let task_deadline: Option<TimeStamp> = match deadline_check {
                    true => {
                        let input_deadline: NaiveDate =
                            date_input_prompt("Task Deadline:").unwrap();
                        Some(TimeStamp::convert(input_deadline))
                    }
                    false => None,
                };

                let task_item: TaskItem =
                    TaskItem::new(task_name, task_description, task_deadline, task_priority)
                        .unwrap();
                task_item.write_to_file().unwrap();
                boards.add_to_board(task_item.task_id.clone(), task_item.task_status).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });

                let mut subtasks_list: Vec<String> = Vec::new();
                loop {
                    let subtask_check: bool =
                        confirm_prompt("Do you want to create subtasks for this task?", None)
                            .unwrap();

                    if subtask_check == true {
                        let subtask_name: String =
                            text_input_prompt("Subtask Name:", None).unwrap();
                        let subtask_description: String =
                            text_input_prompt("Subtask Description:", None).unwrap();
                        let subtask_priority: TaskPriority =
                            select_prompt("Subtask Priority:").unwrap();
                        let deadline_check: bool = confirm_prompt(
                            "Is there a deadline for this subtask?",
                            Some("It's recommended to set a deadline to track for completion."),
                        )
                        .unwrap();

                        let subtask_deadline: Option<TimeStamp> = match deadline_check {
                            true => {
                                let input_deadline: NaiveDate =
                                    date_input_prompt("Subtask Deadline:").unwrap();
                                Some(TimeStamp::convert(input_deadline))
                            }
                            false => None,
                        };

                        let subtask_item: SubTaskItem = SubTaskItem::new(
                            subtask_name,
                            subtask_description,
                            subtask_deadline,
                            subtask_priority,
                        )
                        .unwrap();
                        subtask_item.write_to_file().unwrap_or_else(|err| {
                            println!("{}", err.to_string());
                        });
                        subtasks_list.push(subtask_item.subtask_id.clone());
                        boards.add_to_board(
                            subtask_item.subtask_id.clone(),
                            subtask_item.subtask_status,
                        ).unwrap_or_else(|err| {
                            println!("{}", err.to_string());
                        });
                    } else {
                        break;
                    }
                }

                tasks_link.add_new_link(task_item.task_id.clone(), &subtasks_list).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["add", "subtask"] => {
                let subtask_name: String = text_input_prompt("Subtask Name:", None).unwrap();
                let subtask_description: String =
                    text_input_prompt("Subtask Description:", None).unwrap();
                let subtask_priority: TaskPriority = select_prompt("Subtask Priority:").unwrap();
                let deadline_check: bool = confirm_prompt(
                    "Is there a deadline for this subtask?",
                    Some("It's recommended to set a deadline to track for completion."),
                )
                .unwrap();

                let subtask_deadline: Option<TimeStamp> = match deadline_check {
                    true => {
                        let input_deadline: NaiveDate =
                            date_input_prompt("Subtask Deadline:").unwrap();
                        Some(TimeStamp::convert(input_deadline))
                    }
                    false => None,
                };

                let task_id: String =
                    tasks_select_prompt("Select task id to link to:", &boards).unwrap();
                let subtask_item: SubTaskItem = SubTaskItem::new(
                    subtask_name,
                    subtask_description,
                    subtask_deadline,
                    subtask_priority,
                )
                .unwrap();
                subtask_item.write_to_file().unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
                boards.add_to_board(subtask_item.subtask_id.clone(), subtask_item.subtask_status).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
                tasks_link.add_new_link(task_id, &vec![subtask_item.subtask_id]).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["edit", "task", task_id] => {
                let mut task_item: TaskItem = TaskItem::get_task(&task_id.to_string()).unwrap();
                let task_description: String =
                    text_input_prompt("Task Description:", Some(&task_item.task_description[..]))
                        .unwrap();
                let task_priority: TaskPriority = select_prompt("Task Priority:").unwrap();
                let task_deadline: Option<TimeStamp> = match task_item.task_deadline {
                    Some(_) => {
                        let deadline_check: bool = confirm_prompt(
                            "Do you want to change the deadline for this task?",
                            Some("It's recommended to set a deadline to track for completion."),
                        )
                        .unwrap();

                        match deadline_check {
                            true => {
                                let input_deadline: NaiveDate =
                                    date_input_prompt("Task Deadline:").unwrap();
                                Some(TimeStamp::convert(input_deadline))
                            }
                            false => task_item.task_deadline,
                        }
                    }
                    None => {
                        let deadline_check: bool = confirm_prompt(
                            "Is there a deadline for this task?",
                            Some("It's recommended to set a deadline to track for completion."),
                        )
                        .unwrap();

                        match deadline_check {
                            true => {
                                let input_deadline: NaiveDate =
                                    date_input_prompt("Task Deadline:").unwrap();
                                Some(TimeStamp::convert(input_deadline))
                            }
                            false => None,
                        }
                    }
                };

                let mut subtasks_list: Vec<String> = Vec::new();
                loop {
                    let subtask_check: bool =
                        confirm_prompt("Do you want to create subtasks for this task?", None)
                            .unwrap();

                    if subtask_check == true {
                        let subtask_name: String =
                            text_input_prompt("Subtask Name:", None).unwrap();
                        let subtask_description: String =
                            text_input_prompt("Subtask Description:", None).unwrap();
                        let subtask_priority: TaskPriority =
                            select_prompt("Subtask Priority:").unwrap();
                        let deadline_check: bool = confirm_prompt(
                            "Is there a deadline for this subtask?",
                            Some("It's recommended to set a deadline to track for completion."),
                        )
                        .unwrap();

                        let subtask_deadline: Option<TimeStamp> = match deadline_check {
                            true => {
                                let input_deadline: NaiveDate =
                                    date_input_prompt("Subtask Deadline:").unwrap();
                                Some(TimeStamp::convert(input_deadline))
                            }
                            false => None,
                        };

                        let subtask_item: SubTaskItem = SubTaskItem::new(
                            subtask_name,
                            subtask_description,
                            subtask_deadline,
                            subtask_priority,
                        )
                        .unwrap();
                        subtask_item.write_to_file().unwrap_or_else(|err| {
                            println!("{}", err.to_string());
                        });
                        subtasks_list.push(subtask_item.subtask_id.clone());
                        boards.add_to_board(
                            subtask_item.subtask_id.clone(),
                            subtask_item.subtask_status,
                        ).unwrap_or_else(|err| {
                            println!("{}", err.to_string());
                        });
                    } else {
                        break;
                    }
                }

                tasks_link.add_new_link(task_item.task_id.clone(), &subtasks_list).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
                task_item.task_description = task_description;
                task_item.task_priority = task_priority;
                task_item.task_deadline = task_deadline;
                task_item.write_to_file().unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["edit", "subtask", subtask_id] => {
                let mut subtask_item: SubTaskItem =
                    SubTaskItem::get_task(&subtask_id.to_string()).unwrap();
                let subtask_description: String = text_input_prompt(
                    "Subtask Description:",
                    Some(&subtask_item.subtask_description[..]),
                )
                .unwrap();
                let subtask_priority: TaskPriority = select_prompt("Subtask Priority:").unwrap();

                let subtask_deadline: Option<TimeStamp> = match subtask_item.subtask_deadline {
                    Some(_) => {
                        let deadline_check: bool = confirm_prompt(
                            "Do you want to change the deadline for this subtask?",
                            Some("It's recommended to set a deadline to track for completion."),
                        )
                        .unwrap();

                        match deadline_check {
                            true => {
                                let input_deadline: NaiveDate =
                                    date_input_prompt("Subtask Deadline:").unwrap();
                                Some(TimeStamp::convert(input_deadline))
                            }
                            false => subtask_item.subtask_deadline,
                        }
                    }
                    None => {
                        let deadline_check: bool = confirm_prompt(
                            "Is there a deadline for this subtask?",
                            Some("It's recommended to set a deadline to track for completion."),
                        )
                        .unwrap();

                        match deadline_check {
                            true => {
                                let input_deadline: NaiveDate =
                                    date_input_prompt("Subtask Deadline:").unwrap();
                                Some(TimeStamp::convert(input_deadline))
                            }
                            false => None,
                        }
                    }
                };

                let link_check: bool = confirm_prompt(
                    "Do you want to link this subtask to a different parent task?",
                    None,
                )
                .unwrap();

                if link_check == true {
                    let subtask_id = subtask_item.subtask_id.clone();
                    let current_task_id: String = tasks_link.get_task_id(&subtask_id).unwrap();
                    let new_task_id: String =
                        tasks_select_prompt("Select task id to link to:", &boards).unwrap();
                    tasks_link
                        .update_link(subtask_id, current_task_id, new_task_id)
                        .unwrap();
                }

                subtask_item.subtask_description = subtask_description;
                subtask_item.subtask_priority = subtask_priority;
                subtask_item.subtask_deadline = subtask_deadline;
                subtask_item.write_to_file().unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["link", "subtask", subtask_id] => {
                let current_task_id: String =
                    tasks_link.get_task_id(&subtask_id.to_string()).unwrap();
                let new_task_id: String =
                    tasks_select_prompt("Select task id to link to:", &boards).unwrap();
                tasks_link
                    .update_link(subtask_id.to_string(), current_task_id, new_task_id)
                    .unwrap();
            }
            ["move", "task", task_id, swimlane] => {
                let task_item: TaskItem = TaskItem::get_task(&task_id.to_string()).unwrap();
                match boards.update_board(task_id.to_string(), task_item.task_status, swimlane) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                };

                TaskItem::change_swimlane(&task_id.to_string(), swimlane).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["move", "subtask", subtask_id, swimlane] => {
                let subtask_item: SubTaskItem =
                    SubTaskItem::get_task(&subtask_id.to_string()).unwrap();
                match boards.update_board(
                    subtask_id.to_string(),
                    subtask_item.subtask_status,
                    swimlane,
                ) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                };

                SubTaskItem::change_swimlane(&subtask_id.to_string(), swimlane).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["open", "task", task_id] => {
                match TaskItem::check_if_file_exists(&task_id.to_string()).unwrap() {
                    true => {
                        println!("{}", TaskItem::show_task(&task_id.to_string()).unwrap());
                    }
                    false => println!("{}\n", AppError::TaskNotFound(task_id.to_string())),
                }
            }
            ["open", "subtask", subtask_id] => {
                match SubTaskItem::check_if_file_exists(&subtask_id.to_string()).unwrap() {
                    true => {
                        println!("{}", SubTaskItem::show_task(&subtask_id.to_string()).unwrap());
                    }
                    false => println!("{}\n", AppError::TaskNotFound(subtask_id.to_string())),
                }
            }
            ["delete", "task", task_id] => {
                let task_item: TaskItem = TaskItem::get_task(&task_id.to_string()).unwrap();
                let subtasks_list: Vec<String> = tasks_link.get_subtasks_list(&task_id.to_string());

                for subtask_id in subtasks_list {
                    let subtask_item: SubTaskItem =
                        SubTaskItem::get_task(&subtask_id.to_string()).unwrap();
                    boards.delete_task(subtask_id.to_string(), subtask_item.subtask_status).unwrap_or_else(|err| {
                        println!("{}", err.to_string());
                    });
                    SubTaskItem::delete_task(&subtask_id.to_string()).unwrap_or_else(|err| {
                        println!("{}", err.to_string());
                    });
                    tasks_link.delete_subtask(subtask_id.to_string()).unwrap_or_else(|err| {
                        println!("{}", err.to_string());
                    });
                }

                boards.delete_task(task_id.to_string(), task_item.task_status).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
                TaskItem::delete_task(&task_id.to_string()).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
                tasks_link.delete_task(&task_id.to_string()).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["delete", "subtask", subtask_id] => {
                let subtask_item: SubTaskItem =
                    SubTaskItem::get_task(&subtask_id.to_string()).unwrap();
                boards.delete_task(subtask_id.to_string(), subtask_item.subtask_status).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
                SubTaskItem::delete_task(&subtask_id.to_string()).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
                tasks_link.delete_subtask(subtask_id.to_string()).unwrap_or_else(|err| {
                    println!("{}", err.to_string());
                });
            }
            ["show", "task", swimlane] => {
                match boards.show_tasks(swimlane) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                };
            }
            ["show", "subtask", swimlane] => {
                match boards.show_subtasks(swimlane) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                };
            }
            ["filter", "due", keyword] => {
                match boards.filter_deadline(keyword) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                };
            }
            ["filter", "priority", keyword] => {
                match boards.filter_priority(keyword) {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                };
            }
            ["exit"] => {
                break;
            }
            _ => {
                println!("{}", AppError::InvalidCommand(user_input));
            }
        }
    }
}
