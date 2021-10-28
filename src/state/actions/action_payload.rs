use serde::{Deserialize, Serialize};

use super::action_type::ActionType;

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub enum ActionPayload {
    Add(String),
    Clear,
    Edit(String, String),
    List,
    ListWithStatus(bool),
    Remove(String),
    Set(String, bool),
    Other(String),
}

impl ActionPayload {
    pub fn get_action_type(&self) -> ActionType {
        match self {
            ActionPayload::Add(_) => ActionType::Add,
            ActionPayload::Clear => ActionType::Clear,
            ActionPayload::Edit(_, _) => ActionType::Edit,
            ActionPayload::List => ActionType::List,
            ActionPayload::ListWithStatus(_) => ActionType::ListType,
            ActionPayload::Remove(_) => ActionType::Remove,
            ActionPayload::Set(_, _) => ActionType::Set,
            ActionPayload::Other(_) => ActionType::Other,
        }
    }

    pub fn input_cmd_string(&self) -> String {
        self.get_action_type().get_input_string()
    }

    pub fn expected_arg_count(&self) -> usize {
        self.get_action_type().get_arg_count()
    }
}