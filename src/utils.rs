use crate::{TaskItem, error::AppError};

const APP_DIR_PATH: &str = ".rustic_boards";
const ACTIVE_TASKS_PATH: &str = ".active_tasks";
const ARCHIVE_TASKS_PATH: &str = ".archived_tasks";

fn get_users_home_dir() -> Result<String, AppError> {
    match home::home_dir() {
        Some(path) => return Ok(path.display().to_string()),
        None => return Err(AppError::HomeDirectoryInaccessibleError("Unable to determine user's home directory.".to_string())),
    }
}

fn create_app_dirs() -> Result<String, AppError> {
    let home_dir: String = get_users_home_dir()?;
    let app_dir_path: String = format!("{}\\{}", home_dir, APP_DIR_PATH);

    match std::fs::create_dir_all(&app_dir_path) {
        Ok(_) => {},
        Err(e) => return Err(AppError::HomeDirectoryPermissionError(format!("{} - {}", app_dir_path, e.to_string())))
    };
    match std::fs::create_dir_all(format!("{}\\{}", app_dir_path, ACTIVE_TASKS_PATH)) {
        Ok(_) => {},
        Err(e) => return Err(AppError::HomeDirectoryPermissionError(format!("{} - {}", ACTIVE_TASKS_PATH, e.to_string())))
    };
    match std::fs::create_dir_all(format!("{}\\{}", app_dir_path, ARCHIVE_TASKS_PATH)) {
        Ok(_) => {},
        Err(e) => return Err(AppError::HomeDirectoryPermissionError(format!("{} - {}", ARCHIVE_TASKS_PATH, e.to_string())))
    };

    Ok(app_dir_path)
}

pub fn write_to_file(task_id: String, task_item: TaskItem, archive: bool) -> Result<(), AppError> {
    let app_dir: String = create_app_dirs()?;
    let file_path: String = match archive {
        true => format!("{}\\{}\\{}.bin", app_dir, ARCHIVE_TASKS_PATH, task_id),
        false => format!("{}\\{}\\{}.bin", app_dir, ACTIVE_TASKS_PATH, task_id)
    };
    let json_data: String = match serde_json::to_string_pretty(&task_item) {
        Ok(s) => s,
        Err(e) => return Err(AppError::JSONSerializationError(e.to_string()))
    };
    let bin_data: Vec<u8> = match bincode::serialize(&json_data) {
        Ok(s) => s,
        Err(e) => return Err(AppError::BinarySerializationError(e.to_string()))
    };
    match std::fs::write(&file_path, bin_data) {
        Ok(_) => {},
        Err(e) => return Err(AppError::FileWriteError(format!("{} - {}", file_path, e.to_string())))
    };
    Ok(())
}
