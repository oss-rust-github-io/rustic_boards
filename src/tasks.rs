//! Defines the structure for a task item along with associated helper methods

use crate::{
    constants::{ACTIVE_TASKS_PATH, DIGITS_IN_TASK_ID},
    error::AppError,
    links::TaskToSubtaskMap,
    utils::create_app_dirs,
    notes::TaskNotes,
    TaskPriority, TaskStatus, TimeStamp,
};
use cli_table::{Table, TableDisplay};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Rust structure for a task item
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskItem {
    /// Task ID
    pub task_id: String,

    /// Task Name
    pub task_name: String,

    /// Description for the task
    pub task_description: String,

    /// Date when task got created
    pub task_added_on: TimeStamp,

    /// Date when work started on the task
    pub task_started_on: Option<TimeStamp>,

    /// Date when task is supposed to finish
    pub task_deadline: Option<TimeStamp>,

    /// Date when task got completed
    pub task_completed_on: Option<TimeStamp>,

    /// Status of the task
    pub task_status: TaskStatus,

    /// Priority of the task
    pub task_priority: TaskPriority,
}

impl TaskItem {
    /// Create new task item
    pub fn new(
        task_name: String,
        task_description: String,
        task_deadline: Option<TimeStamp>,
        task_priority: TaskPriority,
    ) -> Result<TaskItem, AppError> {
        let current_timestamp_ms: String =
            match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Ok(s) => s.as_millis().to_string(),
                Err(e) => return Err(AppError::CurrentDateTimeFetchError(e.to_string())),
            };

        let task_id: String = format!(
            "TASK-{}",
            current_timestamp_ms[current_timestamp_ms.len() - DIGITS_IN_TASK_ID..].to_string()
        );

        let task_item: TaskItem = TaskItem {
            task_id: task_id.clone(),
            task_name,
            task_description,
            task_added_on: TimeStamp::new(),
            task_started_on: None,
            task_deadline: task_deadline,
            task_completed_on: None,
            task_status: TaskStatus::ToDo,
            task_priority,
        };
        Ok(task_item)
    }

    /// Fetch task information for given Task ID
    pub fn get_task(task_id: &String) -> Result<Self, AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, task_id);
        let data: Vec<u8> = match std::fs::read(file_path) {
            Ok(s) => s,
            Err(e) => {
                return Err(AppError::FileReadError(format!(
                    "{} - {}",
                    ACTIVE_TASKS_PATH,
                    e.to_string()
                )))
            }
        };
        let task_item: TaskItem = match bincode::deserialize(&data) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinaryDeserializationError(e.to_string())),
        };
        Ok(task_item)
    }

    /// Show all information for given Task ID
    pub fn show_task(task_id: &String) -> Result<(), AppError> {
        let tasks_link: TaskToSubtaskMap = TaskToSubtaskMap::load_from_file()?;
        let task_notes: TaskNotes = TaskNotes::load_from_file()?;
        let task_item: TaskItem = TaskItem::get_task(&task_id.to_string())?;
        let task_added_on: String = task_item
            .task_added_on
            .to_naivedate()?
            .format("%b %e, %Y")
            .to_string();
        let task_started_on: String = match task_item.task_started_on {
            Some(s) => s.to_naivedate()?.format("%b %e, %Y").to_string(),
            None => "None".to_string(),
        };
        let task_deadline: String = match task_item.task_deadline {
            Some(s) => s.to_naivedate()?.format("%b %e, %Y").to_string(),
            None => "None".to_string(),
        };
        let task_completed_on: String = match task_item.task_completed_on {
            Some(s) => s.to_naivedate()?.format("%b %e, %Y").to_string(),
            None => "None".to_string(),
        };
        let subtasks_list: Vec<String> = tasks_link.get_subtasks_list(task_id);
        let notes_list: Vec<String> = task_notes.get_notes(task_id.to_string());

        let display_vec: Vec<Vec<String>> = vec![
            vec!["Task ID".to_string(), task_item.task_id],
            vec!["Task Name".to_string(), task_item.task_name],
            vec!["Task Description".to_string(), task_item.task_description],
            vec!["Task Added On".to_string(), task_added_on],
            vec!["Task Started On".to_string(), task_started_on],
            vec!["Task Deadline".to_string(), task_deadline],
            vec!["Task Completed On".to_string(), task_completed_on],
            vec!["Task Status".to_string(), task_item.task_status.to_string()],
            vec![
                "Task Priority".to_string(),
                task_item.task_priority.to_string(),
            ],
            vec![
                "Subtasks".to_string(),
                subtasks_list
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ],
        ];

        let display_table: TableDisplay = match display_vec.table().display() {
            Ok(s) => s,
            Err(e) => return Err(AppError::TableDisplayParseError(e.to_string())),
        };
        println!("{}", display_table);

        if notes_list.len() == 0 {
            println!("No notes found.");
        } else {
            println!("Additional Notes:");
            for (idx, note) in notes_list.iter().enumerate() {
                println!("{}) {}", idx+1, note);
            }
        }

        Ok(())
    }

    /// Move given Task ID from one swimlane to another in Kanban Board
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

        let mut task_item: TaskItem = TaskItem::get_task(&task_id.to_string())?;

        if (task_item.task_status == TaskStatus::ToDo) && (new_swimlane != TaskStatus::ToDo) {
            task_item.task_started_on = Some(TimeStamp::new());
        }

        if new_swimlane == TaskStatus::Done {
            task_item.task_completed_on = Some(TimeStamp::new());
        }

        task_item.task_status = new_swimlane;
        task_item.write_to_file()?;
        Ok(())
    }

    /// Delete a given Task ID
    pub fn delete_task(task_id: &String) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, task_id);
        match std::fs::remove_file(&file_path) {
            Ok(_) => {}
            Err(e) => {
                return Err(AppError::FileDeleteError(format!(
                    "{} - {}",
                    file_path,
                    e.to_string()
                )))
            }
        };
        Ok(())
    }

    /// Store the task information to a file in disk
    pub fn write_to_file(&self) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, self.task_id);
        let bin_data: Vec<u8> = match bincode::serialize(&self) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinarySerializationError(e.to_string())),
        };
        match std::fs::write(&file_path, bin_data) {
            Ok(_) => {}
            Err(e) => {
                return Err(AppError::FileWriteError(format!(
                    "{} - {}",
                    file_path,
                    e.to_string()
                )))
            }
        };
        Ok(())
    }

    /// Check if the task information file is present in disk for given Task ID
    pub fn check_if_file_exists(task_id: &String) -> Result<bool, AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, task_id);
        Ok(Path::new(&file_path).exists())
    }
}
