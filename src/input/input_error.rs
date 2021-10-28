use serde::{Deserialize, Serialize};

/// The types of errors that can result from an Input Error.
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub enum InputError {
    /// Returned when the command entered was empty or not valid.
    /// Parameter is an optional message containing more detail.
    InvalidCommand(Option<String>),
    /// Returns when the arguments given do not match the expected
    /// arguments for the command requested. Parameter is an optional
    /// message containing more detail.
    InvalidArgument(Option<String>),
}

impl InputError {
    /// Creates an [InputError::InvalidCommand] with a message
    /// explaining that a command cannot be empty.
    pub fn cmd_empty() -> InputError {
        InputError::bad_cmd_str("Command cannot be empty!")
    }

    /// Creates an [InputError::InvalidCommand] with an "Unknown Command"
    /// message. The input should be the command that was entered.
    pub fn cmd_unknown(input: &str) -> InputError {
        InputError::bad_cmd_with(format!("Unknown command {:?}", input))
    }

    /// Creates an [InputError::InvalidArgument] with a message showing
    /// the `expected` arguments for the command vs the `received` arguments.
    pub fn bad_arg_count(received: usize, expected: usize) -> InputError {
        InputError::bad_arg_with(format!(
            "Invalid number of arguments given! Given = {:?} Expected = {:?}",
            received, expected
        ))
    }

    /// A generic empty [InputError::InvalidCommand]
    /// 
    /// ##### *Convenience function for `InputError::InvalidCommand(None)`*
    pub fn bad_cmd() -> InputError {
        InputError::InvalidCommand(None)
    }

    /// Creates an [InputError::InvalidCommand] with the given message.
    /// 
    /// ##### *Convenience function for `InputError::InvalidCommand(Some(String))`*
    pub fn bad_cmd_with(msg: String) -> InputError {
        InputError::InvalidCommand(Some(msg))
    }

    /// Creates an [InputError::InvalidCommand] with the given message.
    /// Calls `to_string` on the given [&str]
    /// 
    /// ##### *Convenience function for `InputError::InvalidCommand(Some(&str))`*
    pub fn bad_cmd_str(msg: &str) -> InputError {
        InputError::InvalidCommand(Some(msg.to_string()))
    }

    /// A generic empty [InputError::InvalidArgument]
    /// 
    /// ##### *Convenience function for `InputError::InvalidArgument(None)`*
    pub fn bad_arg() -> InputError {
        InputError::InvalidArgument(None)
    }

    /// Creates an [InputError::InvalidArgument] with the given message.
    /// 
    /// ##### *Convenience function for `InputError::InvalidArgument(Some(String))`*
    pub fn bad_arg_with(msg: String) -> InputError {
        InputError::InvalidArgument(Some(msg))
    }

    /// Creates an [InputError::InvalidArgument] with the given message.
    /// Calls `to_string` on the given [&str]
    /// 
    /// ##### *Convenience function for `InputError::InvalidArgument(Some(&str))`*
    pub fn bad_arg_str(msg: &str) -> InputError {
        InputError::InvalidArgument(Some(msg.to_string()))
    }

    /// Converts this [InputError] into a String form for display.
    pub fn to_string(&self) -> String {
        match self {
            InputError::InvalidCommand(msg) => {
                if let Some(m) = msg {
                    format!("Invalid Command: {}", m)
                } else {
                    "Invalid Command".to_string()
                }
            }
            InputError::InvalidArgument(msg) => {
                if let Some(m) = msg {
                    format!("Invalid Argument: {}", m)
                } else {
                    "Invalid Argument".to_string()
                }
            }
        }
    }
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}