use crate::{
    error::AppError,
    constants::{
        APP_DIR_PATH, 
        ACTIVE_TASKS_PATH, 
        ACTIVE_SUBTASKS_PATH
    }
};
use chrono::prelude::*;
use serde::{ Deserialize, Serialize };

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
    match std::fs::create_dir_all(format!("{}\\{}", app_dir_path, ACTIVE_SUBTASKS_PATH)) {
        Ok(_) => {},
        Err(e) => return Err(AppError::HomeDirectoryPermissionError(
            format!("{} - {}", ACTIVE_SUBTASKS_PATH, e.to_string())))
    };
    Ok(app_dir_path)
}
