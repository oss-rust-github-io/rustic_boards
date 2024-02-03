#[derive(Debug)]
pub enum AppError {
    TextInputPromptError(String),
    ConfirmPromptError(String),
    DateInputPromptError(String),
    HomeDirectoryInaccessibleError(String),
    HomeDirectoryPermissionError(String),
    JSONSerializationError(String),
    BinarySerializationError(String),
    FileWriteError(String),
    CurrentDateTimeFetchError(String)
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::TextInputPromptError(err) => write!(f, "[TextInputPromptError] {}", err),
            AppError::ConfirmPromptError(err) => write!(f, "[ConfirmPromptError] {}", err),
            AppError::DateInputPromptError(err) => write!(f, "[DateInputPromptError] {}", err),
            AppError::HomeDirectoryInaccessibleError(err) => write!(f, "[HomeDirectoryInaccessibleError] {}", err),
            AppError::HomeDirectoryPermissionError(err) => write!(f, "[HomeDirectoryPermissionError] {}", err),
            AppError::JSONSerializationError(err) => write!(f, "[JSONSerializationError] {}", err),
            AppError::BinarySerializationError(err) => write!(f, "[BinarySerializationError] {}", err),
            AppError::FileWriteError(err) => write!(f, "[FileWriteError] {}", err),
            AppError::CurrentDateTimeFetchError(err) => write!(f, "[CurrentDateTimeFetchError] {}", err),
        }
    }
}

impl std::error::Error for AppError {}
