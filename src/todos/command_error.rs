use serde::{Deserialize, Serialize};

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub enum CommandError {
    TodoAlreadyExists,
    TodoNotFound,
    InputInvalid(String),
}

impl CommandError {
    pub fn to_string(&self) -> String {
        match self {
            CommandError::TodoAlreadyExists => "Todo already exists with that name".to_string(),
            CommandError::TodoNotFound => "Todo with that name not found".to_string(),
            CommandError::InputInvalid(msg) => format!("Input invalid, {}", msg),
        }
    }
}