//! Defines the custom error types for the application

/// Custom error types for the application
#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    TextInputPromptError(String),
    ConfirmPromptError(String),
    SelectPromptError(String),
    HomeDirectoryInaccessibleError(String),
    HomeDirectoryPermissionError(String),
    BinarySerializationError(String),
    BinaryDeserializationError(String),
    FileReadError(String),
    FileWriteError(String),
    FileDeleteError(String),
    CurrentDateTimeFetchError(String),
    InvalidSwimlanePassed(String),
    InvalidCommand(String),
    InvalidDeadlineKeyword(String),
    InvalidPriorityKeyword(String),
    TaskNotFound(String),
    NaiveDateConversionError(String),
    TableDisplayParseError(String),
    SwimlaneNotFoundError(String)
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::TextInputPromptError(err) => write!(f, "[TextInputPromptError] {}", err),
            AppError::ConfirmPromptError(err) => write!(f, "[ConfirmPromptError] {}", err),
            AppError::SelectPromptError(err) => write!(f, "[SelectPromptError] {}", err),
            AppError::HomeDirectoryInaccessibleError(err) => {
                write!(f, "[HomeDirectoryInaccessibleError] {}", err)
            }
            AppError::HomeDirectoryPermissionError(err) => {
                write!(f, "[HomeDirectoryPermissionError] {}", err)
            }
            AppError::BinarySerializationError(err) => {
                write!(f, "[BinarySerializationError] {}", err)
            }
            AppError::BinaryDeserializationError(err) => {
                write!(f, "[BinaryDeserializationError] {}", err)
            }
            AppError::FileReadError(err) => write!(f, "[FileReadError] {}", err),
            AppError::FileWriteError(err) => write!(f, "[FileWriteError] {}", err),
            AppError::FileDeleteError(err) => write!(f, "[FileDeleteError] {}", err),
            AppError::CurrentDateTimeFetchError(err) => {
                write!(f, "[CurrentDateTimeFetchError] {}", err)
            }
            AppError::InvalidSwimlanePassed(err) => write!(f, "[InvalidSwimlanePassed] {}", err),
            AppError::InvalidCommand(err) => write!(f, "[InvalidCommand] {}", err),
            AppError::InvalidDeadlineKeyword(err) => write!(f, "[InvalidDeadlineKeyword] {}", err),
            AppError::InvalidPriorityKeyword(err) => write!(f, "[InvalidPriorityKeyword] {}", err),
            AppError::TaskNotFound(err) => write!(f, "[TaskNotFound] {}", err),
            AppError::NaiveDateConversionError(err) => write!(f, "[NaiveDateConversionError] {}", err),
            AppError::TableDisplayParseError(err) => write!(f, "[TableDisplayParseError] {}", err),
            AppError::SwimlaneNotFoundError(err) => write!(f, "[SwimlaneNotFoundError] {}", err),
        }
    }
}

impl std::error::Error for AppError {}
