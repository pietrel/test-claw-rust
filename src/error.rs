use std::fmt;
use crate::tools::ToolCallError;

#[derive(Debug)]
pub enum AppError {
    ApiError(String),
    ToolError(ToolCallError),
    IoError(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ApiError(e) => write!(f, "API error: {}", e),
            AppError::ToolError(e) => write!(f, "Tool error: {:?}", e),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e)
    }
}

impl From<ToolCallError> for AppError {
    fn from(e: ToolCallError) -> Self {
        AppError::ToolError(e)
    }
}
