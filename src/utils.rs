//! Utilities module defining the helper Rust structures and methods for use across other modules

use crate::{
    constants::{ACTIVE_SUBTASKS_PATH, ACTIVE_TASKS_PATH, APP_DIR_PATH},
    error::AppError,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// Rust structure for datetime
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeStamp {
    year: i32,
    month: u32,
    day: u32,
}

impl TimeStamp {
    /// Get current datetime
    pub fn new() -> Self {
        let current_datetime: DateTime<Local> = Local::now();
        TimeStamp {
            year: current_datetime.year(),
            month: current_datetime.month(),
            day: current_datetime.day(),
        }
    }

    /// Convert given input chrono NaiveDate to TimeStamp structure
    pub fn convert(input_date: NaiveDate) -> Self {
        TimeStamp {
            year: input_date.year(),
            month: input_date.month(),
            day: input_date.day(),
        }
    }

    /// Convert given input datetime to chrono NaiveDate
    pub fn to_naivedate(self) -> Result<NaiveDate, AppError> {
        let date = match NaiveDate::from_ymd_opt(self.year, self.month, self.day) {
            Some(s) => s,
            None => return Err(AppError::NaiveDateConversionError(format!("{}/{}/{}", self.day, self.month, self.year)))
        };
        Ok(date)
    }
}

/// Possible task priority values for use in Kanban Board
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TaskPriority {
    High,
    Medium,
    Low,
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

/// Possible task status or swimlane values for use in Kanban Board
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

/// Get users' home directory on Windows OS-based machine
fn get_users_home_dir() -> Result<String, AppError> {
    match home::home_dir() {
        Some(path) => return Ok(path.display().to_string()),
        None => {
            return Err(AppError::HomeDirectoryInaccessibleError(
                "Unable to determine user's home directory.".to_string(),
            ))
        }
    }
}

/// Create new directory based on given path
fn create_dir(dir_path: &String) -> Result<(), AppError> {
    match std::fs::create_dir_all(&dir_path) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(AppError::HomeDirectoryPermissionError(format!(
                "{} - {}",
                dir_path,
                e.to_string()
            )))
        }
    }
}

/// Create all application directories for storing app information
pub fn create_app_dirs() -> Result<String, AppError> {
    let home_dir: String = get_users_home_dir()?;
    let app_dir_path: String = format!("{}\\{}", home_dir, APP_DIR_PATH);
    let tasks_path: String = format!("{}\\{}", &app_dir_path, ACTIVE_TASKS_PATH);
    let subtasks_path: String = format!("{}\\{}", &app_dir_path, ACTIVE_SUBTASKS_PATH);

    create_dir(&app_dir_path)?;
    create_dir(&tasks_path)?;
    create_dir(&subtasks_path)?;

    Ok(app_dir_path)
}
