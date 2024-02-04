use crate::{
    utils::create_app_dirs,
    constants::KANBAN_BOARD_FILE,
    tasks::TaskItem,
    subtasks::SubTaskItem,
    error::AppError,
    TimeStamp, 
    TaskStatus, 
    TaskPriority,
};
use chrono::{Days, prelude::*};
use std::{collections::HashMap, path::Path};
use serde::{ Deserialize, Serialize };
use cli_table::{Cell, Style, Table, TableDisplay};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct KanbanBoard {
    boards: HashMap<TaskStatus, Vec<String>>
}

impl KanbanBoard {
    pub fn new() -> Self {
        KanbanBoard {
            boards: HashMap::new(),
        }
    }

    pub fn show_tasks(&self, swimlanes: &str) -> Result<(), AppError> {
        let swimlane_to_show: Vec<TaskStatus> = match swimlanes {
            "all" => vec![TaskStatus::ToDo, TaskStatus::InProgress, TaskStatus::Blocked, TaskStatus::InReview, TaskStatus::Done],
            "to-do" => vec![TaskStatus::ToDo],
            "in-progress" => vec![TaskStatus::InProgress],
            "blocked" => vec![TaskStatus::Blocked],
            "in-review" => vec![TaskStatus::InReview],
            "done" => vec![TaskStatus::Done],
            _ => return Err(AppError::InvalidSwimlanePassed(
                format!("{} \nPlease select from following options: \n1) all 2) to-do 3) in-progress 4) blocked 5) in-review 6) done\n", swimlanes.to_string())))
        };

        for swimlane in swimlane_to_show {
            let tasks: &Vec<String> = self.boards.get(&swimlane).unwrap();
            println!("====================");
            println!("{:#?}", swimlane.to_string());
            println!("====================");
            
            let mut display_table: Vec<Vec<String>> = Vec::new();
            for task_id in tasks {
                if TaskItem::check_if_file_exists(task_id).unwrap() == true {
                    let task_item: TaskItem = TaskItem::get_task(task_id).unwrap();
                    let task_deadline: String = match task_item.task_deadline {
                        Some(s) => s.to_naivedate().format("%b %e, %Y").to_string(),
                        None => "None".to_string()
                    };
                    display_table.push(
                        vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline]
                    )
                }
            }

            let display_table: TableDisplay = display_table.table()
                .title(vec![
                    "Task ID".cell().bold(true),
                    "Task Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                ])
                .display().unwrap();
            
            println!("{}", display_table);
        }

        Ok(())
    }

    pub fn show_subtasks(&self, swimlanes: &str) -> Result<(), AppError> {
        let swimlane_to_show: Vec<TaskStatus> = match swimlanes {
            "all" => vec![TaskStatus::ToDo, TaskStatus::InProgress, TaskStatus::Blocked, TaskStatus::InReview, TaskStatus::Done],
            "to-do" => vec![TaskStatus::ToDo],
            "in-progress" => vec![TaskStatus::InProgress],
            "blocked" => vec![TaskStatus::Blocked],
            "in-review" => vec![TaskStatus::InReview],
            "done" => vec![TaskStatus::Done],
            _ => return Err(AppError::InvalidSwimlanePassed(
                format!("{} \nPlease select from following options: \n1) all 2) to-do 3) in-progress 4) blocked 5) in-review 6) done\n", swimlanes.to_string())))
        };

        for swimlane in swimlane_to_show {
            let tasks: &Vec<String> = self.boards.get(&swimlane).unwrap();
            println!("====================");
            println!("{:#?}", swimlane.to_string());
            println!("====================");
            
            let mut display_table: Vec<Vec<String>> = Vec::new();
            for subtask_id in tasks {
                if SubTaskItem::check_if_file_exists(subtask_id).unwrap() == true {
                    let subtask_item: SubTaskItem = SubTaskItem::get_task(subtask_id).unwrap();
                    let subtask_deadline: String = match subtask_item.subtask_deadline {
                        Some(s) => s.to_naivedate().format("%b %e, %Y").to_string(),
                        None => "None".to_string()
                    };
                    display_table.push(
                        vec![subtask_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline]
                    )
                }
            }

            let display_table: TableDisplay = display_table.table()
                .title(vec![
                    "Subtask ID".cell().bold(true),
                    "Subtask Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                ])
                .display().unwrap();
            
            println!("{}", display_table);
        }

        Ok(())
    }

    pub fn filter_deadline(&self, keyword: &str) -> Result<(), AppError> {
        for swimlane in vec![TaskStatus::ToDo, TaskStatus::InProgress, TaskStatus::Blocked, TaskStatus::InReview] {
            let tasks: &Vec<String> = self.boards.get(&swimlane).unwrap();
            println!("====================");
            println!("{:#?}", swimlane.to_string());
            println!("====================");

            let mut display_table: Vec<Vec<String>> = Vec::new();
            for task_id in tasks {
                if TaskItem::check_if_file_exists(task_id).unwrap() == true {
                    let task_item: TaskItem = TaskItem::get_task(task_id).unwrap();

                    if keyword == "no-deadline" {
                        if task_item.task_deadline == None {
                            display_table.push(
                                vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), "None".to_string()]
                            )
                        }
                    } else {
                        if task_item.task_deadline != None {
                            let task_deadline: NaiveDate = task_item.task_deadline.unwrap().to_naivedate();
                            match keyword {
                                "past-deadline" => {
                                    if task_deadline < TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string()]
                                        )
                                    }
                                },
                                "today" => {
                                    if task_deadline == TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string()]
                                        )
                                    }
                                },
                                "tomorrow" => {
                                    if task_deadline == TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string()]
                                        )
                                    }
                                },
                                "after-tomorrow" => {
                                    if task_deadline > TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string()]
                                        )
                                    }
                                },
                                _ => return Err(AppError::InvalidDeadlineKeyword (
                                    format!("{} \nPlease select from following options: \n1) past-deadline 2) today 3) tomorrow 4) after-tomorrow 5) no-deadline\n", keyword)))
                            }
                        }
                    }
                }

                if SubTaskItem::check_if_file_exists(task_id).unwrap() == true {
                    let subtask_item: SubTaskItem = SubTaskItem::get_task(task_id).unwrap();

                    if keyword == "no-deadline" {
                        if subtask_item.subtask_deadline == None {
                            display_table.push(
                                vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), "None".to_string()]
                            )
                        }
                    } else {
                        if subtask_item.subtask_deadline != None {
                            let subtask_deadline: NaiveDate = subtask_item.subtask_deadline.unwrap().to_naivedate();
                            match keyword {
                                "past-deadline" => {
                                    if subtask_deadline < TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string()]
                                        )
                                    }
                                },
                                "today" => {
                                    if subtask_deadline == TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string()]
                                        )
                                    }
                                },
                                "tomorrow" => {
                                    if subtask_deadline == TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string()]
                                        )
                                    }
                                },
                                "after-tomorrow" => {
                                    if subtask_deadline > TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string()]
                                        )
                                    }
                                },
                                _ => return Err(AppError::InvalidDeadlineKeyword (
                                    format!("{} \nPlease select from following options: \n1) past-deadline 2) today 3) tomorrow 4) after-tomorrow 5) no-deadline\n", keyword)))
                            }
                        }
                    }
                }
            }

            let display_table: TableDisplay = display_table.table()
                .title(vec![
                    "Task ID".cell().bold(true),
                    "Task Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                ])
                .display().unwrap();
            
            println!("{}", display_table);
        }

        Ok(())
    }

    pub fn filter_priority(&self, keyword: &str) -> Result<(), AppError> {
        for swimlane in vec![TaskStatus::ToDo, TaskStatus::InProgress, TaskStatus::Blocked, TaskStatus::InReview] {
            let tasks: &Vec<String> = self.boards.get(&swimlane).unwrap();
            println!("====================");
            println!("{:#?}", swimlane.to_string());
            println!("====================");

            let mut display_table: Vec<Vec<String>> = Vec::new();
            for task_id in tasks {
                if TaskItem::check_if_file_exists(task_id).unwrap() == true {
                    let task_item: TaskItem = TaskItem::get_task(task_id).unwrap();
                    let task_deadline: String = match task_item.task_deadline {
                        Some(s) => s.to_naivedate().format("%b %e, %Y").to_string(),
                        None => "None".to_string()
                    };

                    match keyword {
                        "high" => {
                            if task_item.task_priority == TaskPriority::High {
                                display_table.push(
                                    vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string()]
                                )
                            }
                        },
                        "medium" => {
                            if task_item.task_priority == TaskPriority::Medium {
                                display_table.push(
                                    vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string()]
                                )
                            }
                        },
                        "low" => {
                            if task_item.task_priority == TaskPriority::Low {
                                display_table.push(
                                    vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string()]
                                )
                            }
                        },
                        _ => return Err(AppError::InvalidPriorityKeyword (
                            format!("{} \nPlease select from following options: \n1) high 2) medium 3) low\n", keyword)))
                    }
                }

                if SubTaskItem::check_if_file_exists(task_id).unwrap() == true {
                    let subtask_item: SubTaskItem = SubTaskItem::get_task(task_id).unwrap();
                    let subtask_deadline: String = match subtask_item.subtask_deadline {
                        Some(s) => s.to_naivedate().format("%b %e, %Y").to_string(),
                        None => "None".to_string()
                    };

                    match keyword {
                        "high" => {
                            if subtask_item.subtask_priority == TaskPriority::High {
                                display_table.push(
                                    vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string()]
                                )
                            }
                        },
                        "medium" => {
                            if subtask_item.subtask_priority == TaskPriority::Medium {
                                display_table.push(
                                    vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string()]
                                )
                            }
                        },
                        "low" => {
                            if subtask_item.subtask_priority == TaskPriority::Low {
                                display_table.push(
                                    vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string()]
                                )
                            }
                        },
                        _ => return Err(AppError::InvalidPriorityKeyword (
                            format!("{} \nPlease select from following options: \n1) high 2) medium 3) low\n", keyword)))
                    }
                }
            }

            let display_table: TableDisplay = display_table.table()
                .title(vec![
                    "Task ID".cell().bold(true),
                    "Task Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                ])
                .display().unwrap();
            
            println!("{}", display_table);
        }

        Ok(())
    }

    pub fn add_to_board(&mut self, task_id: String, swimlane: TaskStatus) {
        let mut tasks_list: Vec<String> = match self.boards.get(&swimlane) {
            Some(s) => s.to_vec(),
            None => Vec::new()
        };
        tasks_list.push(task_id);
        self.boards.insert(swimlane, tasks_list);
        self.write_to_file().unwrap();
    }

    pub fn update_board(&mut self, task_id: String, current_swimlane: TaskStatus, new_swimlane: &str) -> Result<(), AppError> {
        let new_swimlane: TaskStatus = match new_swimlane {
            "to-do" => TaskStatus::ToDo,
            "in-progress" => TaskStatus::InProgress,
            "blocked" => TaskStatus::Blocked,
            "in-review" => TaskStatus::InReview,
            "done" => TaskStatus::Done,
            _ => return Err(AppError::InvalidSwimlanePassed(
                format!("{} \nPlease select from following options: \n1) to-do 2) in-progress 3) blocked 4) in-review 5) done\n", new_swimlane.to_string())))
        };
        
        let mut tasks_list: Vec<String> = self.boards.get(&current_swimlane).unwrap().to_vec();
        let index: usize = tasks_list.iter().position(|x| *x == task_id).unwrap();
        tasks_list.remove(index);
        self.boards.insert(current_swimlane, tasks_list);

        let mut tasks_list: Vec<String> = self.boards.get(&new_swimlane).unwrap().to_vec();
        tasks_list.push(task_id);
        self.boards.insert(new_swimlane, tasks_list);
        self.write_to_file().unwrap();

        Ok(())
    }

    pub fn delete_task(&mut self, task_id: String, swimlane: TaskStatus) {
        let mut tasks_list: Vec<String> = self.boards.get(&swimlane).unwrap().to_vec();
        let index: usize = tasks_list.iter().position(|x| *x == task_id).unwrap();
        tasks_list.remove(index);
        self.boards.insert(swimlane, tasks_list);
        self.write_to_file().unwrap();
    }

    pub fn load_from_file() -> Result<Self, AppError> {
        let app_dir: String = create_app_dirs()?;
        let data: Vec<u8> = match std::fs::read(format!("{}\\{}", app_dir, KANBAN_BOARD_FILE)) {
            Ok(s) => s,
            Err(e) => return Err(AppError::FileReadError(
                format!("{} - {}", KANBAN_BOARD_FILE, e.to_string())))
        };
        let boards: KanbanBoard = match bincode::deserialize(&data) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinaryDeserializationError(e.to_string()))
        };
        Ok(boards)
    }

    pub fn write_to_file(&self) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let data: Vec<u8> = match bincode::serialize(&self) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinarySerializationError(e.to_string()))
        };
        match std::fs::write(format!("{}\\{}", app_dir, KANBAN_BOARD_FILE), data) {
            Ok(_) => {},
            Err(e) => return Err(AppError::FileWriteError(
                format!("{} - {}", KANBAN_BOARD_FILE, e.to_string())))
        };
        Ok(())
    }

    pub fn check_if_file_exists() -> Result<bool, AppError> {
        let app_dir: String = create_app_dirs()?;
        let boards_file: String = format!("{}\\{}", app_dir, KANBAN_BOARD_FILE);
        Ok(Path::new(&boards_file).exists())
    }
}
