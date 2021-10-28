use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::action_payload::ActionPayload;

use crate::{
    input::{action_argument::ActionArgument, input_error::InputError},
    utils::general::string_to_bool,
};

/// The types of Actions that can be done to a [TodoList]

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ActionType {
    Add,
    Clear,
    Edit,
    List,
    ListType,
    Remove,
    Set,
    Other,
}

impl ActionType {
    pub fn try_parse_cmd(s: &str) -> Result<Self, InputError> {
        match s {
            "add" => Ok(ActionType::Add),
            "clear" => Ok(ActionType::Clear),
            "edit" => Ok(ActionType::Edit),
            "ls" => Ok(ActionType::List),
            "lss" => Ok(ActionType::ListType),
            "rm" => Ok(ActionType::Remove),
            "set" => Ok(ActionType::Set),
            "" => Err(InputError::cmd_empty()),
            _ => Err(InputError::cmd_unknown(s)),
        }
    }

    pub fn try_parse_name(s: &str) -> Result<Self, InputError> {
        match s {
            "Add" => Ok(ActionType::Add),
            "Clear" => Ok(ActionType::Clear),
            "Edit" => Ok(ActionType::Edit),
            "List" => Ok(ActionType::List),
            "ListType" => Ok(ActionType::ListType),
            "Remove" => Ok(ActionType::Remove),
            "Set" => Ok(ActionType::Set),
            _ => Err(InputError::bad_cmd_with(format!(
                "Unknown action type {:?}",
                s
            ))),
        }
    }

    pub fn try_create_payload(&self, args: &Vec<String>) -> Result<ActionPayload, InputError> {
        if self.get_arg_count() != args.len() {
            return Err(self.arg_count_error(args.len()));
        }

        match self {
            ActionType::Add => {
                if let Some(add_value) = args.first() {
                    if add_value.is_empty() {
                        Err(InputError::bad_arg())
                    } else {
                        Ok(ActionPayload::Add(add_value.clone()))
                    }
                } else {
                    Err(InputError::bad_arg_str("Unable to add empty todo."))
                }
            }
            ActionType::Clear => Ok(ActionPayload::Clear),
            ActionType::Edit => {
                let existing = args.first();
                let editted = args.last();

                if existing.is_none() || editted.is_none() {
                    return Err(InputError::bad_arg_str("Edit must be provided two arguments, the existing todo to edit and the new text it should be changed to."));
                }

                let ex_unw = existing.unwrap();
                let ed_unw = editted.unwrap();

                if ex_unw.is_empty() || ed_unw.is_empty() {
                    return Err(InputError::bad_arg_str(
                        "Edit cannot be passed empty strings",
                    ));
                }

                Ok(ActionPayload::Edit(ex_unw.clone(), ed_unw.clone()))
            }
            ActionType::List => Ok(ActionPayload::List),
            ActionType::ListType => {
                if let Some(lss_value_raw) = args.first() {
                    if let Some(lss_value) = string_to_bool(lss_value_raw) {
                        Ok(ActionPayload::ListWithStatus(lss_value))
                    } else {
                        Err(InputError::bad_arg_with(format!(
                            "Unable to parse {:?} to valid boolean value.",
                            lss_value_raw
                        )))
                    }
                } else {
                    Err(InputError::bad_arg())
                }
            }
            ActionType::Remove => {
                if let Some(rm_value) = args.first() {
                    if rm_value.is_empty() {
                        Err(InputError::bad_arg())
                    } else {
                        Ok(ActionPayload::Remove(rm_value.clone()))
                    }
                } else {
                    Err(InputError::bad_arg())
                }
            }
            ActionType::Set => {
                if let (Some(set_key), Some(set_value_raw)) = (args.first(), args.last()) {
                    if let Some(set_value) = string_to_bool(set_value_raw) {
                        Ok(ActionPayload::Set(set_key.clone(), set_value))
                    } else {
                        Err(InputError::bad_arg_with(format!(
                            "Unable to parse {:?} to valid boolean value.",
                            set_value_raw
                        )))
                    }
                } else {
                    Err(InputError::bad_arg())
                }
            }
            ActionType::Other => Ok(ActionPayload::Other(args.join(" "))),
        }
    }

    pub fn get_arguments(&self) -> Vec<ActionArgument> {
        match self {
            ActionType::Add => vec![ActionArgument::string("todo", 0)],
            ActionType::Clear => vec![],
            ActionType::Edit => vec![
                ActionArgument::existing("todo", 0),
                ActionArgument::string("new text", 1),
            ],
            ActionType::List => vec![],
            ActionType::ListType => vec![ActionArgument::boolean("status", 0)],
            ActionType::Remove => vec![ActionArgument::existing("todo", 0)],
            ActionType::Set => vec![
                ActionArgument::existing("todo", 0),
                ActionArgument::boolean("status", 1),
            ],
            ActionType::Other => vec![ActionArgument::string("input", 0)],
        }
    }

    pub fn get_action_name(&self) -> String {
        match self {
            ActionType::Add => "Add".to_string(),
            ActionType::Clear => "Clear".to_string(),
            ActionType::Edit => "Edit".to_string(),
            ActionType::List => "List".to_string(),
            ActionType::ListType => "ListType".to_string(),
            ActionType::Remove => "Remove".to_string(),
            ActionType::Set => "Set".to_string(),
            ActionType::Other => "Other".to_string(),
        }
    }

    pub fn get_input_string(&self) -> String {
        match self {
            ActionType::Add => "add".to_string(),
            ActionType::Clear => "clear".to_string(),
            ActionType::Edit => "edit".to_string(),
            ActionType::List => "ls".to_string(),
            ActionType::ListType => "lss".to_string(),
            ActionType::Remove => "rm".to_string(),
            ActionType::Set => "set".to_string(),
            ActionType::Other => "secret".to_string(),
        }
    }

    pub fn get_arg_count(&self) -> usize {
        match self {
            ActionType::Add => 1,
            ActionType::Clear => 0,
            ActionType::Edit => 2,
            ActionType::List => 0,
            ActionType::ListType => 1,
            ActionType::Remove => 1,
            ActionType::Set => 2,
            ActionType::Other => 1,
        }
    }

    pub fn arg_count_error(&self, input_count: usize) -> InputError {
        InputError::InvalidArgument(Some(format!("Invalid argument count - the {:?} command expects {:?} argument{}, but {:?} {} received.", self.get_input_string(), self.get_arg_count(), if self.get_arg_count() > 1 { "s" } else { "" }, input_count, if input_count == 1 { "was" } else { "were" })))
    }

    pub fn all_actions() -> Vec<Self> {
        vec![
            ActionType::Add,
            ActionType::Clear,
            ActionType::Edit,
            ActionType::List,
            ActionType::ListType,
            ActionType::Remove,
            ActionType::Set,
            ActionType::Other,
        ]
    }

    pub fn all_action_names() -> Vec<String> {
        ActionType::all_actions()
            .iter_mut()
            .map(|at| at.get_action_name())
            .collect()
    }
}
