//! Defines the constant values for the application

/// Number of digits to keep when generating Task or SubTask IDs
pub const DIGITS_IN_TASK_ID: usize = 5;

/// Application directory path
pub const APP_DIR_PATH: &str = ".rustic_boards";

/// Directory to store all tasks information
pub const ACTIVE_TASKS_PATH: &str = ".tasks";

/// Directory to store all subtasks information
pub const ACTIVE_SUBTASKS_PATH: &str = ".subtasks";

/// File to store all Kanban Board information
pub const KANBAN_BOARD_FILE: &str = "boards.bin";

/// File to store all task-to-subtask link information
pub const TAKS_LINK_FILE: &str = "tasks_link.bin";
