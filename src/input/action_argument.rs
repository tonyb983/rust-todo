use serde::{Deserialize, Serialize};

use super::argument_type::ArgumentType;

/// This struct describes an argument for an action that can be performed
/// by the user. Assists in validating and presenting the user with useful
/// information when prompting them for input.
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub struct ActionArgument {
    pub name: String,
    pub arg_type: ArgumentType,
    pub order: usize,
}

impl ActionArgument {
    /// Validate the input against this argument type. Forwards
    /// call to [self.arg_type.validate].
    pub fn validate<S: AsRef<str>>(&self, input: S) -> bool {
        self.arg_type.validate(input)
    }

    /// Convenience function to create an [ArgumentType::ExistingTodo].
    pub fn existing<S: Into<String>>(name: S, order: usize) -> Self {
        ActionArgument {
            name: name.into(),
            arg_type: ArgumentType::ExistingTodo,
            order,
        }
    }

    /// Convenience function to create an [ArgumentType::String].
    pub fn string<S: Into<String>>(name: S, order: usize) -> Self {
        ActionArgument {
            name: name.into(),
            arg_type: ArgumentType::String,
            order,
        }
    }

    /// Convenience function to create an [ArgumentType::Boolean].
    pub fn boolean<S: Into<String>>(name: S, order: usize) -> Self {
        ActionArgument {
            name: name.into(),
            arg_type: ArgumentType::Boolean,
            order,
        }
    }

    // pub fn number<S: Into<String>>(name: S, order: usize) -> Self {
    //     ActionArgument {
    //         name: name.into(),
    //         arg_type: ArgumentType::Number,
    //         order,
    //     }
    // }
}
