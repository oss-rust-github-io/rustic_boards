//! Defines the structure for a subtask item along with associated helper methods

use crate::{
    constants::{ACTIVE_SUBTASKS_PATH, DIGITS_IN_TASK_ID},
    error::AppError,
    links::TaskToSubtaskMap,
    utils::create_app_dirs,
    TaskPriority, TaskStatus, TimeStamp,
};
use cli_table::{Table, TableDisplay};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Rust structure for a subtask item
#[derive(Debug, Serialize, Deserialize)]
pub struct SubTaskItem {
    /// SubTask ID
    pub subtask_id: String,

    /// SubTask Name
    pub subtask_name: String,

    /// Description for the subtask
    pub subtask_description: String,

    /// Date when subtask got created
    pub subtask_added_on: TimeStamp,

    /// Date when work started on the subtask
    pub subtask_started_on: Option<TimeStamp>,

    /// Date when subtask is supposed to finish
    pub subtask_deadline: Option<TimeStamp>,

    /// Date when subtask got completed
    pub subtask_completed_on: Option<TimeStamp>,

    /// Status of the subtask
    pub subtask_status: TaskStatus,

    /// Priority of the subtask
    pub subtask_priority: TaskPriority,
}

impl SubTaskItem {
    /// Create new subtask item
    pub fn new(
        subtask_name: String,
        subtask_description: String,
        subtask_deadline: Option<TimeStamp>,
        subtask_priority: TaskPriority,
    ) -> Result<SubTaskItem, AppError> {
        let current_timestamp_ms: String =
            match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                Ok(s) => s.as_millis().to_string(),
                Err(e) => return Err(AppError::CurrentDateTimeFetchError(e.to_string())),
            };

        let task_id: String = format!(
            "SUBTASK-{}",
            current_timestamp_ms[current_timestamp_ms.len() - DIGITS_IN_TASK_ID..].to_string()
        );

        let task_item: SubTaskItem = SubTaskItem {
            subtask_id: task_id.clone(),
            subtask_name,
            subtask_description,
            subtask_added_on: TimeStamp::new(),
            subtask_started_on: None,
            subtask_deadline: subtask_deadline,
            subtask_completed_on: None,
            subtask_status: TaskStatus::ToDo,
            subtask_priority,
        };
        Ok(task_item)
    }

    /// Fetch subtask information for given SubTask ID
    pub fn get_task(subtask_id: &String) -> Result<Self, AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String =
            format!("{}\\{}\\{}.bin", app_dir, ACTIVE_SUBTASKS_PATH, subtask_id);
        let data: Vec<u8> = match std::fs::read(file_path) {
            Ok(s) => s,
            Err(e) => {
                return Err(AppError::FileReadError(format!(
                    "{} - {}",
                    ACTIVE_SUBTASKS_PATH,
                    e.to_string()
                )))
            }
        };
        let task_item: SubTaskItem = match bincode::deserialize(&data) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinaryDeserializationError(e.to_string())),
        };
        Ok(task_item)
    }

    /// Show all information for given SubTask ID
    pub fn show_task(subtask_id: &String) -> Result<TableDisplay, AppError> {
        let tasks_link: TaskToSubtaskMap = TaskToSubtaskMap::load_from_file()?;
        let subtask_item: SubTaskItem = SubTaskItem::get_task(&subtask_id.to_string())?;
        let subtask_added_on: String = subtask_item
            .subtask_added_on
            .to_naivedate()?
            .format("%b %e, %Y")
            .to_string();
        let subtask_started_on: String = match subtask_item.subtask_started_on {
            Some(s) => s.to_naivedate()?.format("%b %e, %Y").to_string(),
            None => "None".to_string(),
        };
        let subtask_deadline: String = match subtask_item.subtask_deadline {
            Some(s) => s.to_naivedate()?.format("%b %e, %Y").to_string(),
            None => "None".to_string(),
        };
        let subtask_completed_on: String = match subtask_item.subtask_completed_on {
            Some(s) => s.to_naivedate()?.format("%b %e, %Y").to_string(),
            None => "None".to_string(),
        };
        let task_id: String = match tasks_link.get_task_id(subtask_id) {
            Some(s) => s,
            None => {
                return Err(AppError::TaskNotFound(format!(
                    "No parent task found for subtask {}",
                    subtask_id
                )))
            }
        };
        let display_vec: Vec<Vec<String>> = vec![
            vec!["Subtask ID".to_string(), subtask_item.subtask_id],
            vec!["Subtask Name".to_string(), subtask_item.subtask_name],
            vec![
                "Subtask Description".to_string(),
                subtask_item.subtask_description,
            ],
            vec!["Subtask Added On".to_string(), subtask_added_on],
            vec!["Subtask Started On".to_string(), subtask_started_on],
            vec!["Subtask Deadline".to_string(), subtask_deadline],
            vec!["Subtask Completed On".to_string(), subtask_completed_on],
            vec![
                "Subtask Status".to_string(),
                subtask_item.subtask_status.to_string(),
            ],
            vec![
                "Subtask Priority".to_string(),
                subtask_item.subtask_priority.to_string(),
            ],
            vec!["Parent Task".to_string(), task_id],
        ];

        let display_table: TableDisplay = match display_vec.table().display() {
            Ok(s) => s,
            Err(e) => return Err(AppError::TableDisplayParseError(e.to_string())),
        };

        Ok(display_table)
    }

    /// Move given SubTask ID from one swimlane to another in Kanban Board
    pub fn change_swimlane(subtask_id: &String, swimlane: &str) -> Result<(), AppError> {
        let new_swimlane: TaskStatus = match swimlane {
            "to-do" => TaskStatus::ToDo,
            "in-progress" => TaskStatus::InProgress,
            "blocked" => TaskStatus::Blocked,
            "in-review" => TaskStatus::InReview,
            "done" => TaskStatus::Done,
            _ => return Err(AppError::InvalidSwimlanePassed(
                format!("{} \nPlease select from following options: \n1) to-do 2) in-progress 3) blocked 4) in-review 5) done\n", swimlane.to_string())))
        };

        let mut subtask_item: SubTaskItem = SubTaskItem::get_task(&subtask_id.to_string())?;

        if (subtask_item.subtask_status == TaskStatus::ToDo) && (new_swimlane != TaskStatus::ToDo) {
            subtask_item.subtask_started_on = Some(TimeStamp::new());
        }

        if new_swimlane == TaskStatus::Done {
            subtask_item.subtask_completed_on = Some(TimeStamp::new());
        }

        subtask_item.subtask_status = new_swimlane;
        subtask_item.write_to_file()?;
        Ok(())
    }

    /// Delete a given SubTask ID
    pub fn delete_task(subtask_id: &String) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String =
            format!("{}\\{}\\{}.bin", app_dir, ACTIVE_SUBTASKS_PATH, subtask_id);
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

    /// Store the subtask information to a file in disk
    pub fn write_to_file(&self) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!(
            "{}\\{}\\{}.bin",
            app_dir, ACTIVE_SUBTASKS_PATH, self.subtask_id
        );
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

    /// Check if the subtask information file is present in disk for given SubTask ID
    pub fn check_if_file_exists(task_id: &String) -> Result<bool, AppError> {
        let app_dir: String = create_app_dirs()?;
        let file_path: String = format!("{}\\{}\\{}.bin", app_dir, ACTIVE_SUBTASKS_PATH, task_id);
        Ok(Path::new(&file_path).exists())
    }
}
