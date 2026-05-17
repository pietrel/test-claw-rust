use crate::tools::Tool;
use async_trait::async_trait;
use std::fmt::Display;

use console::style;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use super::ToolArgParsingError;

pub struct Args {
    file_path: String,
}

impl TryFrom<serde_json::Value> for Args {
    type Error = ToolArgParsingError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let file_path = value["file_path"]
            .as_str()
            .ok_or(ToolArgParsingError {})?
            .to_string();

        Ok(Self { file_path })
    }
}

pub struct ReadTool {
    args: Args,
}

impl ReadTool {
    pub fn new(
        args: impl TryInto<Args, Error = ToolArgParsingError>,
    ) -> Result<Self, ToolArgParsingError> {
        Ok(Self {
            args: args.try_into()?,
        })
    }
}

#[async_trait]
impl Tool for ReadTool {
    async fn call(&self) -> String {
        let mut content = String::new();

        if let Ok(mut file) = File::open(&self.args.file_path).await {
            let _ = file.read_to_string(&mut content).await;
        }

        content
    }
}

impl Display for ReadTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", style("Read").cyan().bold(), self.args.file_path)
    }
}
