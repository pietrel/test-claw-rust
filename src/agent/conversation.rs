use serde_json::{Value, json};

pub struct Conversation {
    messages: Vec<Value>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn add_user_message(&mut self, content: String) {
        self.messages.push(json!({
            "role": "user",
            "content": content
        }));
    }

    pub fn add_message(&mut self, message: Value) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<Value> {
        &self.messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user_message() {
        let mut conv = Conversation::new();
        conv.add_user_message("Hello".to_string());
        assert_eq!(conv.get_messages().len(), 1);
        assert_eq!(conv.get_messages()[0]["role"], "user");
        assert_eq!(conv.get_messages()[0]["content"], "Hello");
    }
}
