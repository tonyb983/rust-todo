use std::str::FromStr;
use serde::{Deserialize, Serialize};

use crate::utils::general::string_to_bool;

/// The types of arguments that can be accepted by an action.
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub enum ArgumentType {
    /// Represents a boolean input argument. Using [`string_to_bool`] is the best
    /// way to convert from an input [String] to a boolean as it converts to lower-
    /// case first, and also accounts for typical *human* responses to a yes or no 
    /// question.
    Boolean,
    /// Represents a [String] input argument. The only validation necessary is that it
    /// is not empty.
    String,
    /// Represents an existing item in the Todo Database. Useful for such commands as
    /// [crate::state::actions::ActionType::Remove]
    ExistingTodo,
}

impl ArgumentType {
    /// Validates the given String-like `S` against whatever [ArgumentType] 
    /// this `self` represents.
    pub fn validate<S: AsRef<str>>(&self, input: S) -> bool {
        // let value: String = input.into();
        match self {
            ArgumentType::Boolean => {
                if let Some(_) = string_to_bool(input) {
                    true
                } else {
                    false
                }
            }
            ArgumentType::String => {
                if input.as_ref().is_empty() {
                    false
                } else {
                    true
                }
            }
            // TODO Fixing this is probably going to mean redesigning a lot of things.
            ArgumentType::ExistingTodo => {
                if input.as_ref().is_empty() {
                    false
                } else {
                    true
                }
            }
        }
    }
}
