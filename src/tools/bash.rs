use crate::tools::Tool;
use async_trait::async_trait;
use std::fmt::Display;

use super::ToolArgParsingError;
use console::style;
use serde_json::Value;

pub struct Args {
    command: String,
}

impl TryFrom<Value> for Args {
    type Error = ToolArgParsingError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let command = value["command"]
            .as_str()
            .ok_or(ToolArgParsingError {})?
            .to_string();
        Ok(Self { command })
    }
}

pub struct BashTool {
    args: Args,
}

impl BashTool {
    pub fn new(
        args: impl TryInto<Args, Error = ToolArgParsingError>,
    ) -> Result<Self, ToolArgParsingError> {
        Ok(Self {
            args: args.try_into()?,
        })
    }
}

#[async_trait]
impl Tool for BashTool {
    async fn call(&self) -> String {
        let command_str = self.args.command.clone();
        let result = tokio::task::spawn_blocking(move || {
            std::process::Command::new("bash")
                .arg("-c")
                .arg(command_str)
                .output()
        })
        .await
        .ok();

        match result {
            Some(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let code = output.status.code().unwrap_or(-1);
                format!(
                    "stdout:\n{}\nstderr:\n{}\nreturn code: {}",
                    stdout.trim_end(),
                    stderr.trim_end(),
                    code
                )
            }
            Some(Err(e)) => format!("Failed to execute command: {}", e),
            None => "Failed to spawn blocking task".to_string(),
        }
    }
}

impl Display for BashTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", style("Bash").cyan().bold(), self.args.command)
    }
}
