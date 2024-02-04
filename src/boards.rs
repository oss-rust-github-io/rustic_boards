use crate::{
    constants::KANBAN_BOARD_FILE, error::AppError, links::TaskToSubtaskMap, subtasks::SubTaskItem,
    tasks::TaskItem, utils::create_app_dirs, TaskPriority, TaskStatus, TimeStamp,
};
use chrono::{prelude::*, Days};
use cli_table::{Cell, Style, Table, TableDisplay};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct KanbanBoard {
    boards: HashMap<TaskStatus, Vec<String>>,
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

        let tasks_link: TaskToSubtaskMap = TaskToSubtaskMap::load_from_file().unwrap();

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
                        None => "None".to_string(),
                    };
                    let num_subtasks: String = tasks_link.get_num_subtasks(task_id).to_string();
                    display_table.push(vec![
                        task_id.clone(),
                        task_item.task_name,
                        task_item.task_priority.to_string(),
                        task_deadline,
                        num_subtasks,
                    ])
                }
            }

            let display_table: TableDisplay = display_table
                .table()
                .title(vec![
                    "Task ID".cell().bold(true),
                    "Task Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                    "Subtasks".cell().bold(true),
                ])
                .display()
                .unwrap();

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

        let tasks_link: TaskToSubtaskMap = TaskToSubtaskMap::load_from_file().unwrap();

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
                        None => "None".to_string(),
                    };
                    let task_id: String = match tasks_link.get_task_id(subtask_id) {
                        Some(s) => s,
                        None => "None".to_string(),
                    };
                    display_table.push(vec![
                        subtask_id.clone(),
                        subtask_item.subtask_name,
                        subtask_item.subtask_priority.to_string(),
                        subtask_deadline,
                        task_id,
                    ])
                }
            }

            let display_table: TableDisplay = display_table
                .table()
                .title(vec![
                    "Subtask ID".cell().bold(true),
                    "Subtask Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                    "Parent Task".cell().bold(true),
                ])
                .display()
                .unwrap();

            println!("{}", display_table);
        }

        Ok(())
    }

    pub fn filter_deadline(&self, keyword: &str) -> Result<(), AppError> {
        for swimlane in vec![
            TaskStatus::ToDo,
            TaskStatus::InProgress,
            TaskStatus::Blocked,
            TaskStatus::InReview,
        ] {
            let tasks_link: TaskToSubtaskMap = TaskToSubtaskMap::load_from_file().unwrap();
            let tasks: &Vec<String> = self.boards.get(&swimlane).unwrap();
            println!("====================");
            println!("{:#?}", swimlane.to_string());
            println!("====================");

            let mut display_table: Vec<Vec<String>> = Vec::new();
            for task_id in tasks {
                if TaskItem::check_if_file_exists(task_id).unwrap() == true {
                    let task_item: TaskItem = TaskItem::get_task(task_id).unwrap();
                    let num_subtasks: String = tasks_link.get_num_subtasks(task_id).to_string();

                    if keyword == "no-deadline" {
                        if task_item.task_deadline == None {
                            display_table.push(vec![
                                task_id.clone(),
                                task_item.task_name,
                                task_item.task_priority.to_string(),
                                "None".to_string(),
                                num_subtasks,
                            ])
                        }
                    } else {
                        if task_item.task_deadline != None {
                            let task_deadline: NaiveDate =
                                task_item.task_deadline.unwrap().to_naivedate();
                            match keyword {
                                "past-deadline" => {
                                    if task_deadline < TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string(), num_subtasks]
                                        )
                                    }
                                },
                                "today" => {
                                    if task_deadline == TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string(), num_subtasks]
                                        )
                                    }
                                },
                                "tomorrow" => {
                                    if task_deadline == TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string(), num_subtasks]
                                        )
                                    }
                                },
                                "after-tomorrow" => {
                                    if task_deadline > TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string(), num_subtasks]
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
                    let parent_task_id: String = tasks_link.get_task_id(task_id).unwrap();

                    if keyword == "no-deadline" {
                        if subtask_item.subtask_deadline == None {
                            display_table.push(vec![
                                task_id.clone(),
                                subtask_item.subtask_name,
                                subtask_item.subtask_priority.to_string(),
                                "None".to_string(),
                                parent_task_id,
                            ])
                        }
                    } else {
                        if subtask_item.subtask_deadline != None {
                            let subtask_deadline: NaiveDate =
                                subtask_item.subtask_deadline.unwrap().to_naivedate();
                            match keyword {
                                "past-deadline" => {
                                    if subtask_deadline < TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string(), parent_task_id]
                                        )
                                    }
                                },
                                "today" => {
                                    if subtask_deadline == TimeStamp::new().to_naivedate() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string(), parent_task_id]
                                        )
                                    }
                                },
                                "tomorrow" => {
                                    if subtask_deadline == TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string(), parent_task_id]
                                        )
                                    }
                                },
                                "after-tomorrow" => {
                                    if subtask_deadline > TimeStamp::new().to_naivedate().checked_add_days(Days::new(1)).unwrap() {
                                        display_table.push(
                                            vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string(), parent_task_id]
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

            let display_table: TableDisplay = display_table
                .table()
                .title(vec![
                    "Task ID".cell().bold(true),
                    "Task Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                    "Subtasks/Parent Task".cell().bold(true),
                ])
                .display()
                .unwrap();

            println!("{}", display_table);
        }

        Ok(())
    }

    pub fn filter_priority(&self, keyword: &str) -> Result<(), AppError> {
        for swimlane in vec![
            TaskStatus::ToDo,
            TaskStatus::InProgress,
            TaskStatus::Blocked,
            TaskStatus::InReview,
        ] {
            let tasks_link: TaskToSubtaskMap = TaskToSubtaskMap::load_from_file().unwrap();
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
                        None => "None".to_string(),
                    };
                    let num_subtasks: String = tasks_link.get_num_subtasks(task_id).to_string();

                    match keyword {
                        "high" => {
                            if task_item.task_priority == TaskPriority::High {
                                display_table.push(
                                    vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string(), num_subtasks]
                                )
                            }
                        },
                        "medium" => {
                            if task_item.task_priority == TaskPriority::Medium {
                                display_table.push(
                                    vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string(), num_subtasks]
                                )
                            }
                        },
                        "low" => {
                            if task_item.task_priority == TaskPriority::Low {
                                display_table.push(
                                    vec![task_id.clone(), task_item.task_name, task_item.task_priority.to_string(), task_deadline.to_string(), num_subtasks]
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
                        None => "None".to_string(),
                    };
                    let parent_task_id: String = tasks_link.get_task_id(task_id).unwrap();

                    match keyword {
                        "high" => {
                            if subtask_item.subtask_priority == TaskPriority::High {
                                display_table.push(
                                    vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string(), parent_task_id]
                                )
                            }
                        },
                        "medium" => {
                            if subtask_item.subtask_priority == TaskPriority::Medium {
                                display_table.push(
                                    vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string(), parent_task_id]
                                )
                            }
                        },
                        "low" => {
                            if subtask_item.subtask_priority == TaskPriority::Low {
                                display_table.push(
                                    vec![task_id.clone(), subtask_item.subtask_name, subtask_item.subtask_priority.to_string(), subtask_deadline.to_string(), parent_task_id]
                                )
                            }
                        },
                        _ => return Err(AppError::InvalidPriorityKeyword (
                            format!("{} \nPlease select from following options: \n1) high 2) medium 3) low\n", keyword)))
                    }
                }
            }

            let display_table: TableDisplay = display_table
                .table()
                .title(vec![
                    "Task ID".cell().bold(true),
                    "Task Name".cell().bold(true),
                    "Priority".cell().bold(true),
                    "Deadline".cell().bold(true),
                    "Subtasks/Parent Task".cell().bold(true),
                ])
                .display()
                .unwrap();

            println!("{}", display_table);
        }

        Ok(())
    }

    pub fn get_tasks_list(&self) -> Vec<String> {
        let mut tasks_list: Vec<String> = Vec::new();
        for swimlane in vec![
            TaskStatus::ToDo,
            TaskStatus::InProgress,
            TaskStatus::Blocked,
            TaskStatus::InReview,
        ] {
            let tasks: Vec<String> = self.boards.get(&swimlane).unwrap().to_vec();
            for task_id in tasks {
                if TaskItem::check_if_file_exists(&task_id).unwrap() == true {
                    tasks_list.push(task_id);
                }
            }
        }
        tasks_list
    }

    pub fn add_to_board(&mut self, task_id: String, swimlane: TaskStatus) {
        let mut tasks_list: Vec<String> = match self.boards.get(&swimlane) {
            Some(s) => s.to_vec(),
            None => Vec::new(),
        };
        tasks_list.push(task_id);
        self.boards.insert(swimlane, tasks_list);
        self.write_to_file().unwrap();
    }

    pub fn update_board(
        &mut self,
        task_id: String,
        current_swimlane: TaskStatus,
        new_swimlane: &str,
    ) -> Result<(), AppError> {
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
            Err(e) => {
                return Err(AppError::FileReadError(format!(
                    "{} - {}",
                    KANBAN_BOARD_FILE,
                    e.to_string()
                )))
            }
        };
        let boards: KanbanBoard = match bincode::deserialize(&data) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinaryDeserializationError(e.to_string())),
        };
        Ok(boards)
    }

    pub fn write_to_file(&self) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let data: Vec<u8> = match bincode::serialize(&self) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinarySerializationError(e.to_string())),
        };
        match std::fs::write(format!("{}\\{}", app_dir, KANBAN_BOARD_FILE), data) {
            Ok(_) => {}
            Err(e) => {
                return Err(AppError::FileWriteError(format!(
                    "{} - {}",
                    KANBAN_BOARD_FILE,
                    e.to_string()
                )))
            }
        };
        Ok(())
    }

    pub fn check_if_file_exists() -> Result<bool, AppError> {
        let app_dir: String = create_app_dirs()?;
        let boards_file: String = format!("{}\\{}", app_dir, KANBAN_BOARD_FILE);
        Ok(Path::new(&boards_file).exists())
    }
}
