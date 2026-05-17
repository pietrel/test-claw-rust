use std::fmt::Display;
use async_trait::async_trait;

mod bash;
mod glob;
mod read;
mod write;

#[async_trait]
pub trait Tool: Display + Send + Sync {
    async fn call(&self) -> String;
}

pub type BoxedTool = Box<dyn Tool>;

pub fn get_tools() -> Vec<serde_json::Value> {
    let tools = vec![
        serde_json::from_str(include_str!("./read.json")).unwrap(),
        serde_json::from_str(include_str!("./write.json")).unwrap(),
        serde_json::from_str(include_str!("./glob.json")).unwrap(),
        serde_json::from_str(include_str!("./bash.json")).unwrap(),
    ];
    tools
}

pub struct ToolArgParsingError;

#[derive(Debug)]
pub enum ToolCallError {
    NotFound,
    JsonParseError,
    ArgParseError,
}

impl From<ToolArgParsingError> for ToolCallError {
    fn from(_value: ToolArgParsingError) -> Self {
        ToolCallError::ArgParseError
    }
}

pub fn create_tool(name: &str, arguments: &str) -> Result<BoxedTool, ToolCallError> {
    let args = serde_json::from_str::<serde_json::Value>(arguments)
        .map_err(|_| ToolCallError::JsonParseError)?;

    Ok(match name {
        "Read" => Box::new(read::ReadTool::new(args)?),
        "Write" => Box::new(write::WriteTool::new(args)?),
        "Glob" => Box::new(glob::GlobTool::new(args)?),
        "Bash" => Box::new(bash::BashTool::new(args)?),
        _ => return Err(ToolCallError::NotFound),
    })
}
