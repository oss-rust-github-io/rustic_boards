//! Defines the Notes structure along with associated helper methods

use crate::{
    constants::NOTES_FILE, 
    error::AppError,
    utils::create_app_dirs
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

/// Rust structure for storing notes for Tasks and SubTasks
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TaskNotes {
    /// Using HashMaps to store list of notes for different Tasks and SubTasks
    notes: HashMap<String, Vec<String>>,
}

impl TaskNotes {
    /// Create new blank notes (for first time setup)
    pub fn new() -> Self {
        TaskNotes {
            notes: HashMap::new(),
        }
    }

    /// Add a new note for given Task or SubTask IDs
    pub fn add_new_note(
        &mut self,
        task_id: String,
        notes_list: Vec<String>,
    ) -> Result<(), AppError> {
        let mut current_notes_list: Vec<String> = match self.notes.get(&task_id) {
            Some(s) => s.to_vec(),
            None => Vec::new(),
        };
        current_notes_list.extend(notes_list.clone());
        self.notes.insert(task_id, current_notes_list);
        self.write_to_file()?;
        Ok(())
    }

    pub fn get_notes(&self, task_id: String) -> Vec<String> {
        let notes_list: Vec<String> = match self.notes.get(&task_id) {
            Some(s) => s.to_vec(),
            None => Vec::new(),
        };
        notes_list
    }

    /// Load the Notes information from stored file in disk
    pub fn load_from_file() -> Result<Self, AppError> {
        let app_dir: String = create_app_dirs()?;
        let data: Vec<u8> = match std::fs::read(format!("{}\\{}", app_dir, NOTES_FILE)) {
            Ok(s) => s,
            Err(e) => {
                return Err(AppError::FileReadError(format!(
                    "{} - {}",
                    NOTES_FILE,
                    e.to_string()
                )))
            }
        };
        let task_notes: TaskNotes = match bincode::deserialize(&data) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinaryDeserializationError(e.to_string())),
        };
        Ok(task_notes)
    }

    /// Store the Notes informationto a file in disk
    pub fn write_to_file(&self) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let data: Vec<u8> = match bincode::serialize(&self) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinarySerializationError(e.to_string())),
        };
        match std::fs::write(format!("{}\\{}", app_dir, NOTES_FILE), data) {
            Ok(_) => {}
            Err(e) => {
                return Err(AppError::FileWriteError(format!(
                    "{} - {}",
                    NOTES_FILE,
                    e.to_string()
                )))
            }
        };
        Ok(())
    }

    /// Check if the Notes file is present in disk
    pub fn check_if_file_exists() -> Result<bool, AppError> {
        let app_dir: String = create_app_dirs()?;
        let notes_file: String = format!("{}\\{}", app_dir, NOTES_FILE);
        Ok(Path::new(&notes_file).exists())
    }
}
