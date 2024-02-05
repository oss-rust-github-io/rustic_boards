//! Defines the different user input prompts for interacting with the application

use crate::{boards::KanbanBoard, error::AppError, utils::TaskPriority};
use chrono::prelude::{Local, NaiveDate};
use inquire::{
    formatter::DEFAULT_DATE_FORMATTER, ui::RenderConfig, validator::Validation, Confirm,
    CustomType, Select, Text,
};

/// Standard text prompt that returns the user string input
pub fn text_input_prompt(message: &str, default: Option<&str>) -> Result<String, AppError> {
    let input: String = match default {
        Some(s) => match Text::new(message).with_default(s).prompt() {
            Ok(s) => s,
            Err(e) => return Err(AppError::TextInputPromptError(e.to_string())),
        },
        None => match Text::new(message).prompt() {
            Ok(s) => s,
            Err(e) => return Err(AppError::TextInputPromptError(e.to_string())),
        },
    };
    Ok(input)
}

/// Confirm prompt to ask the user for simple yes/no questions
pub fn confirm_prompt(message: &str, help_message: Option<&str>) -> Result<bool, AppError> {
    let input: bool = match (Confirm {
        message,
        default: Some(false),
        placeholder: Some("yes|no"),
        help_message,
        formatter: &|ans| match ans {
            true => "yes".to_owned(),
            false => "no".to_owned(),
        },
        parser: &|ans| match ans {
            "yes" => Ok(true),
            "no" => Ok(false),
            _ => Err(()),
        },
        error_message: "Reply with 'yes' or 'no'".into(),
        default_value_formatter: &|def| match def {
            true => String::from("yes"),
            false => String::from("no"),
        },
        render_config: RenderConfig::default(),
    })
    .prompt()
    {
        Ok(s) => s,
        Err(e) => return Err(AppError::ConfirmPromptError(e.to_string())),
    };
    Ok(input)
}

/// Date input prompt to ask the user for task or subtask deadlines
pub fn date_input_prompt(message: &str) -> Result<NaiveDate, AppError> {
    let input = match CustomType::<NaiveDate>::new(message)
        .with_placeholder("dd/mm/yyyy")
        .with_parser(&|i| NaiveDate::parse_from_str(i, "%d/%m/%Y").map_err(|_e| ()))
        .with_formatter(DEFAULT_DATE_FORMATTER)
        .with_error_message("Please enter a valid date in dd/mm/yyyy format.")
        .with_validator(|val: &NaiveDate| {
            if val < &Local::now().date_naive().into() {
                Ok(Validation::Invalid(
                    "Entered date cannot be prior to current date.".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()
    {
        Ok(s) => s,
        Err(e) => return Err(AppError::ConfirmPromptError(e.to_string())),
    };
    Ok(input)
}

/// Select prompt to ask the user to select one option among different task priorities
pub fn select_prompt(message: &str) -> Result<TaskPriority, AppError> {
    let task_priority: TaskPriority = match Select::new(message, task_priority()).prompt() {
        Ok(s) => s,
        Err(e) => return Err(AppError::SelectPromptError(e.to_string())),
    };
    Ok(task_priority)
}

/// Select prompt to ask the user to select one option among different Task IDs
pub fn tasks_select_prompt(message: &str, boards: &KanbanBoard) -> Result<String, AppError> {
    let tasks_list: String = match Select::new(message, boards.get_tasks_list()?).prompt() {
        Ok(s) => s,
        Err(e) => return Err(AppError::SelectPromptError(e.to_string())),
    };
    Ok(tasks_list)
}

fn task_priority() -> Vec<TaskPriority> {
    vec![TaskPriority::Low, TaskPriority::Medium, TaskPriority::High]
}
