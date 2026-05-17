use crate::tools::Tool;
use async_trait::async_trait;
use std::fmt::Display;

use super::ToolArgParsingError;
use console::style;
use glob::glob;

pub struct Args {
    pattern: String,
}

impl TryFrom<serde_json::Value> for Args {
    type Error = ToolArgParsingError;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let pattern = value["pattern"]
            .as_str()
            .ok_or(ToolArgParsingError {})?
            .to_string();
        Ok(Self { pattern })
    }
}

pub struct GlobTool {
    args: Args,
}

impl GlobTool {
    pub fn new(
        args: impl TryInto<Args, Error = ToolArgParsingError>,
    ) -> Result<Self, ToolArgParsingError> {
        Ok(Self {
            args: args.try_into()?,
        })
    }
}

#[async_trait]
impl Tool for GlobTool {
    async fn call(&self) -> String {
        let mut matches: Vec<String> = Vec::new();
        for entry in glob(&self.args.pattern).unwrap() {
            match entry {
                Ok(path) => {
                    // Convert to string path
                    if let Some(path_str) = path.to_str() {
                        matches.push(path_str.to_string());
                    }
                }
                Err(_) => (),
            }
        }
        matches.join("\n")
    }
}

impl Display for GlobTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", style("Glob").cyan().bold(), self.args.pattern)
    }
}
