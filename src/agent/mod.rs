pub mod conversation;

use crate::ui::consent::{self, Consent};
use crate::ui::spinner;
use crate::tools::{self, get_tools};
use crate::error::AppError;
use async_openai::{Client, config::OpenAIConfig};
use console::style;
use conversation::Conversation;
use serde_json::{Value, json};

pub struct AgentExecutor {
    client: Client<OpenAIConfig>,
    model: String,
    yes_to_all: bool,
}

impl AgentExecutor {
    pub fn new(client: Client<OpenAIConfig>, model: String, yes_to_all: bool) -> Self {
        Self {
            client,
            model,
            yes_to_all,
        }
    }

    pub async fn run(&self, prompt: String) -> Result<(), AppError> {
        let mut conversation = Conversation::new();
        conversation.add_user_message(prompt);

        let tools = get_tools();

        loop {
            let response_result = spinner::spin_with(
                "Thinking...",
                "Thought for",
                self.client.chat().create_byot(json!({
                    "messages": conversation.get_messages(),
                    "model": self.model,
                    "tools": tools
                })),
            )
            .await;

            let mut response: Value = response_result.map_err(|e| AppError::ApiError(e.to_string()))?;

            log::debug!("{}", response);

            let message = response["choices"][0]["message"].take();
            conversation.add_message(message.clone());

            if let Some(tool_calls) = message["tool_calls"].as_array() {
                for call in tool_calls {
                    let id = call["id"].as_str().expect("Wrong format");
                    let name = call["function"]["name"].as_str().expect("Wrong format");
                    let arguments = call["function"]["arguments"]
                        .as_str()
                        .expect("Wrong format");

                    match tools::create_tool(name, arguments) {
                        Ok(tool) => {
                            eprintln!("{} {}", style(">").yellow().bold(), tool);

                            if let Some(agreement) = if self.yes_to_all {
                                Some(Consent::Yes)
                            } else {
                                consent::ask("Proceed with tool execution?")?
                            } {
                                match agreement {
                                    Consent::Yes => {
                                        let tool_response = spinner::spin_with(
                                            &format!("Executing {}", style(name).cyan().bold()),
                                            &format!("Executed {} in", style(name).cyan().bold()),
                                            async { tool.call().await },
                                        )
                                        .await;

                                        conversation.add_message(json!({
                                            "role": "tool",
                                            "tool_call_id": id,
                                            "content": tool_response
                                        }));
                                    }
                                    Consent::No => {
                                        conversation.add_message(json!({
                                            "role": "tool",
                                            "tool_call_id": id,
                                            "content": "User declined to execute this tool."
                                        }));
                                    }
                                    Consent::Reason(reason) => {
                                        conversation.add_message(json!({
                                            "role": "tool",
                                            "tool_call_id": id,
                                            "content": format!("User declined to execute this tool. Reason: {reason}")
                                        }));
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Tool creation error: {:?}", err);
                        }
                    }
                }
            } else if let Some(content) = message["content"].as_str() {
                println!("{}", content);
                break;
            }

            if response["choices"][0]["finish_reason"].as_str() == Some("stop") {
                break;
            }
        }

        Ok(())
    }
}
