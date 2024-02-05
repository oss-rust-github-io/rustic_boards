use crate::{constants::TAKS_LINK_FILE, error::AppError, utils::create_app_dirs};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TaskToSubtaskMap {
    tasks: HashMap<String, Vec<String>>,
}

impl TaskToSubtaskMap {
    pub fn new() -> Self {
        TaskToSubtaskMap {
            tasks: HashMap::new(),
        }
    }

    pub fn get_task_id(&self, subtask_id: &String) -> Option<String> {
        let mut parent_task_id: Option<String> = None;
        for (task_id, subtasks_list) in &self.tasks {
            if subtasks_list.contains(&subtask_id) {
                parent_task_id = Some(task_id.clone());
                break;
            }
        }
        parent_task_id
    }

    pub fn get_subtasks_list(&self, task_id: &String) -> Vec<String> {
        let subtasks_list: Vec<String> = match self.tasks.get(task_id) {
            Some(s) => s.to_vec(),
            None => Vec::new(),
        };
        subtasks_list
    }

    pub fn get_num_subtasks(&self, task_id: &String) -> i32 {
        let num_subtasks: i32 = match self.tasks.get(task_id) {
            Some(s) => s.to_vec().len() as i32,
            None => 0,
        };
        num_subtasks
    }

    pub fn delete_task(&mut self, task_id: &String) -> Result<(), AppError> {
        self.tasks.remove(task_id);
        self.write_to_file()?;
        Ok(())
    }

    pub fn delete_subtask(&mut self, subtask_id: String) -> Result<(), AppError> {
        let mut parent_task_id: String = String::new();
        let mut updated_subtasks_list: Vec<String> = Vec::new();
        
        for (task_id, subtasks_list) in &self.tasks {
            if subtasks_list.contains(&subtask_id) {
                parent_task_id = task_id.to_string();
                updated_subtasks_list = subtasks_list.to_vec();
                break;
            }
        }

        match updated_subtasks_list
            .iter()
            .position(|x| *x == subtask_id) {
                Some(s) => {
                    updated_subtasks_list.remove(s);
                    self.tasks.insert(parent_task_id, updated_subtasks_list);
                    self.write_to_file()?;
                    return Ok(())
                },
                None => Ok(())
            }
    }

    pub fn add_new_link(&mut self, task_id: String, subtask_list: &Vec<String>) -> Result<(), AppError> {
        let mut current_subtasks_list: Vec<String> = match self.tasks.get(&task_id) {
            Some(s) => s.to_vec(),
            None => Vec::new(),
        };
        current_subtasks_list.extend(subtask_list.clone());
        self.tasks.insert(task_id, current_subtasks_list);
        self.write_to_file()?;
        Ok(())
    }

    pub fn update_link(
        &mut self,
        subtask_id: String,
        current_task_id: String,
        new_task_id: String,
    ) -> Result<(), AppError> {
        let mut subtasks_list: Vec<String> = match self.tasks.get(&current_task_id) {
            Some(s) => s.to_vec(),
            None => return Err(AppError::TaskNotFound(current_task_id)),
        };
        
        match subtasks_list.iter().position(|x| *x == subtask_id) {
            Some(s) => {
                subtasks_list.remove(s);
                self.tasks.insert(current_task_id, subtasks_list);
            },
            None => {}
        };

        let mut subtasks_list: Vec<String> = match self.tasks.get(&new_task_id) {
            Some(s) => s.to_vec(),
            None => return Err(AppError::TaskNotFound(new_task_id)),
        };
        subtasks_list.push(subtask_id);
        self.tasks.insert(new_task_id, subtasks_list);
        self.write_to_file()?;
        Ok(())
    }

    pub fn load_from_file() -> Result<Self, AppError> {
        let app_dir: String = create_app_dirs()?;
        let data: Vec<u8> = match std::fs::read(format!("{}\\{}", app_dir, TAKS_LINK_FILE)) {
            Ok(s) => s,
            Err(e) => {
                return Err(AppError::FileReadError(format!(
                    "{} - {}",
                    TAKS_LINK_FILE,
                    e.to_string()
                )))
            }
        };
        let tasks_link: TaskToSubtaskMap = match bincode::deserialize(&data) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinaryDeserializationError(e.to_string())),
        };
        Ok(tasks_link)
    }

    pub fn write_to_file(&self) -> Result<(), AppError> {
        let app_dir: String = create_app_dirs()?;
        let data: Vec<u8> = match bincode::serialize(&self) {
            Ok(s) => s,
            Err(e) => return Err(AppError::BinarySerializationError(e.to_string())),
        };
        match std::fs::write(format!("{}\\{}", app_dir, TAKS_LINK_FILE), data) {
            Ok(_) => {}
            Err(e) => {
                return Err(AppError::FileWriteError(format!(
                    "{} - {}",
                    TAKS_LINK_FILE,
                    e.to_string()
                )))
            }
        };
        Ok(())
    }

    pub fn check_if_file_exists() -> Result<bool, AppError> {
        let app_dir: String = create_app_dirs()?;
        let boards_file: String = format!("{}\\{}", app_dir, TAKS_LINK_FILE);
        Ok(Path::new(&boards_file).exists())
    }
}
