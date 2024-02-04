mod error;
use error::AppError;
use chrono::{Days, prelude::*};
use std::{collections::HashMap, path::Path};
use serde::{ Deserialize, Serialize };
use cli_table::{Cell, Style, Table, TableDisplay};

const DIGITS_IN_TASK_ID: usize = 5;
const APP_DIR_PATH: &str = ".rustic_boards";
const KANBAN_BOARD_FILE: &str = "boards.bin";
const ACTIVE_TASKS_PATH: &str = ".tasks";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeStamp {
    year: i32,
    month: u32,
    day: u32
}

impl TimeStamp {
    pub fn new() -> Self {
        let current_datetime: DateTime<Local> = Local::now();
        TimeStamp {
            year: current_datetime.year(),
            month: current_datetime.month(),
            day: current_datetime.day()
        }
    }

    pub fn convert(input_date: NaiveDate) -> Self {
        TimeStamp {
            year: input_date.year(),
            month: input_date.month(),
            day: input_date.day()
        }
    }

    pub fn to_naivedate(self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.year, self.month, self.day).unwrap()
    } 
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TaskPriority {
    High,
    Medium,
    Low
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::High => write!(f, "High"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::Low => write!(f, "Low"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Blocked,
    InReview,
    Done,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::ToDo => write!(f, "To-Do"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Blocked => write!(f, "Blocked"),
            TaskStatus::InReview => write!(f, "In Review"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskItem {
    pub task_id: String,
    pub task_name: String,
    pub task_description: String,
    pub task_added_on: TimeStamp,
    pub task_started_on: Option<TimeStamp>,
    pub task_deadline: Option<TimeStamp>,
    pub task_completed_on: Option<TimeStamp>,
    pub task_status: TaskStatus,
    pub task_priority: TaskPriority
}

impl TaskItem {
    pub fn new (
        task_name: String, 
        task_description: String, 
        task_deadline: Option<TimeStamp>,
        task_priority: TaskPriority
    ) -> Result<TaskItem, AppError> {
        let current_timestamp_ms: String = match std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH) {
                Ok(s) => s.as_millis().to_string(),
                Err(e) => return Err(AppError::CurrentDateTimeFetchError(e.to_string()))
            };

        let task_id: String = format!("TASK-{}", current_timestamp_ms[
            current_timestamp_ms.len() - DIGITS_IN_TASK_ID..].to_string());

        let task_item: TaskItem = TaskItem {
            task_id: task_id.clone(),
            task_name,
            task_description,
            task_added_on: TimeStamp::new(),
            task_started_on: None,
            task_deadline: task_deadline,
            task_completed_on: None,
            task_status: TaskStatus::ToDo,
            task_priority
        };
        Ok(task_item)
    }

    pub fn get_task(task_id: &String) -> Result<Self, AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, task_id);
        let data: Vec<u8> = match std::fs::read(file_path) {
            Ok(s) => s,
            Err(e) => return Err(AppError::FileReadError(
                format!("{} - {}", ACTIVE_TASKS_PATH, e.to_string())))
        };
        let task_item: TaskItem = match bincode::deserialize(&data) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinaryDeserializationError(e.to_string()))
        };
        Ok(task_item)
    }

    pub fn show_task(task_id: &String) {
        let task_item: TaskItem = TaskItem::get_task(&task_id.to_string()).unwrap();
        let task_added_on: String = task_item.task_added_on.to_naivedate().format("%b %e, %Y").to_string();
        let task_started_on: String = match task_item.task_started_on {
            Some(s) => s.to_naivedate().format("%b %e, %Y").to_string(),
            None => "None".to_string()
        };
        let task_deadline: String = match task_item.task_deadline {
            Some(s) => s.to_naivedate().format("%b %e, %Y").to_string(),
            None => "None".to_string()
        };
        let task_completed_on: String = match task_item.task_completed_on {
            Some(s) => s.to_naivedate().format("%b %e, %Y").to_string(),
            None => "None".to_string()
        };
        let display_table: TableDisplay = vec![
            vec!["Task ID".to_string(), task_item.task_id],
            vec!["Task Name".to_string(), task_item.task_name],
            vec!["Task Description".to_string(), task_item.task_description],
            vec!["Task Added On".to_string(), task_added_on],
            vec!["Task Started On".to_string(), task_started_on],
            vec!["Task Deadline".to_string(), task_deadline],
            vec!["Task Completed On".to_string(), task_completed_on],
            vec!["Task Status".to_string(), task_item.task_status.to_string()],
            vec!["Task Priority".to_string(), task_item.task_priority.to_string()],
        ].table()
        .display().unwrap();
        
        println!("{}", display_table);
    }

    pub fn change_swimlane(task_id: &String, swimlane: &str) -> Result<(), AppError> {
        let new_swimlane: TaskStatus = match swimlane {
            "to-do" => TaskStatus::ToDo,
            "in-progress" => TaskStatus::InProgress,
            "blocked" => TaskStatus::Blocked,
            "in-review" => TaskStatus::InReview,
            "done" => TaskStatus::Done,
            _ => return Err(AppError::InvalidSwimlanePassed(
                format!("{} \nPlease select from following options: \n1) to-do 2) in-progress 3) blocked 4) in-review 5) done\n", swimlane.to_string())))
        };

        let mut task_item: TaskItem = TaskItem::get_task(&task_id.to_string()).unwrap();

        if (task_item.task_status == TaskStatus::ToDo) && (new_swimlane != TaskStatus::ToDo) {
            task_item.task_started_on = Some(TimeStamp::new());
        }

        if new_swimlane == TaskStatus::Done {
            task_item.task_completed_on = Some(TimeStamp::new());
        }

        task_item.task_status = new_swimlane;
        task_item.write_to_file().unwrap();
        Ok(())
    }

    pub fn delete_task(task_id: &String) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, task_id);
        match std::fs::remove_file(&file_path) {
            Ok(_) => {},
            Err(e) => return Err(AppError::FileDeleteError(
                format!("{} - {}", file_path, e.to_string())))
        };
        Ok(())
    }

    pub fn write_to_file(&self) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, self.task_id);
        let bin_data: Vec<u8> = match bincode::serialize(&self) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinarySerializationError(e.to_string()))
        };
        match std::fs::write(&file_path, bin_data) {
            Ok(_) => {},
            Err(e) => return Err(AppError::FileWriteError(
                format!("{} - {}", file_path, e.to_string())))
        };
        Ok(())
    }

    pub fn check_if_file_exists(task_id: &String) -> Result<bool, AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, task_id);
        Ok(Path::new(&file_path).exists())
    }
}

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

    pub fn show(&self, swimlanes: &str) -> Result<(), AppError> {
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

fn get_users_home_dir() -> Result<String, AppError> {
    match home::home_dir() {
        Some(path) => return Ok(path.display().to_string()),
        None => return Err(AppError::HomeDirectoryInaccessibleError(
            "Unable to determine user's home directory.".to_string())),
    }
}

pub fn create_app_dirs() -> Result<String, AppError> {
    let home_dir: String = get_users_home_dir()?;
    let app_dir_path: String = format!("{}\\{}", home_dir, APP_DIR_PATH);

    match std::fs::create_dir_all(&app_dir_path) {
        Ok(_) => {},
        Err(e) => return Err(AppError::HomeDirectoryPermissionError(
            format!("{} - {}", app_dir_path, e.to_string())))
    };
    match std::fs::create_dir_all(format!("{}\\{}", app_dir_path, ACTIVE_TASKS_PATH)) {
        Ok(_) => {},
        Err(e) => return Err(AppError::HomeDirectoryPermissionError(
            format!("{} - {}", ACTIVE_TASKS_PATH, e.to_string())))
    };
    Ok(app_dir_path)
}
