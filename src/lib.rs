use chrono::prelude::*;

const DIGITS_IN_TASK_ID: usize = 7;

#[allow(dead_code)]
#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Blocked,
    InReview,
    Done,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TaskItem {
    task_id: String,
    task_name: String,
    task_description: String,
    task_added_on: TimeStamp,
    task_started_on: Option<TimeStamp>,
    task_deadline: Option<TimeStamp>,
    task_completed_on: Option<TimeStamp>,
    task_status: TaskStatus,
    has_subtask: bool,
    subtask_id: Option<Vec<String>>,
}

impl TaskItem {
    pub fn new (task_name: String, task_description: String, task_deadline: Option<TimeStamp>) -> Self {
        let current_timestamp_ms: String = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis().to_string();

        let task_id: String = current_timestamp_ms[current_timestamp_ms.len() - DIGITS_IN_TASK_ID..].to_string();

        TaskItem {
            task_id,
            task_name,
            task_description,
            task_added_on: TimeStamp::new(),
            task_started_on: None,
            task_deadline: task_deadline,
            task_completed_on: None,
            task_status: TaskStatus::ToDo,
            has_subtask: false,
            subtask_id: None
        }
    }
}
