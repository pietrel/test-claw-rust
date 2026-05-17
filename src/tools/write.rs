use crate::tools::Tool;
use async_trait::async_trait;
use std::fmt::Display;

use console::style;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use super::ToolArgParsingError;

pub struct Args {
    file_path: String,
    content: String,
}

impl TryFrom<serde_json::Value> for Args {
    type Error = ToolArgParsingError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let file_path = value["file_path"]
            .as_str()
            .ok_or(ToolArgParsingError {})?
            .to_string();

        let content = value["content"]
            .as_str()
            .ok_or(ToolArgParsingError {})?
            .to_string();

        Ok(Self { file_path, content })
    }
}

pub struct WriteTool {
    args: Args,
}

impl WriteTool {
    pub fn new(
        args: impl TryInto<Args, Error = ToolArgParsingError>,
    ) -> Result<Self, ToolArgParsingError> {
        Ok(Self {
            args: args.try_into()?,
        })
    }
}

#[async_trait]
impl Tool for WriteTool {
    async fn call(&self) -> String {
        if let Ok(mut file) = File::create(&self.args.file_path).await {
            let _ = file.write(self.args.content.as_bytes()).await;
        }

        self.args.content.clone()
    }
}

impl Display for WriteTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_content = String::new();

        for (i, line) in self.args.content.split('\n').enumerate() {
            formatted_content.push_str(&format!(
                "{:>3} {}\n",
                style(i).dim(),
                style(line).on_black()
            ));
        }

        write!(
            f,
            "{} {}\n\n{}",
            style("Write").cyan().bold(),
            self.args.file_path,
            formatted_content
        )
    }
}
