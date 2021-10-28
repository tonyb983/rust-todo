use std::io;

use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input, Select};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::action_argument::ActionArgument;
use super::argument_type::ArgumentType;

lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref THEME: ColorfulTheme = ColorfulTheme::default();
}

/// Tri-State object representing a successful prompt response,
/// response cancellation, or an error that has occurred.s
#[derive(Debug)]
pub enum ResponseState<TValue> {
    Value(TValue),
    Cancelled,
    Error(io::Error),
}

impl<TValue> ResponseState<TValue> {
    pub fn error(err: io::Error) -> Self {
        ResponseState::Error(err)
    }

    pub fn cancelled() -> Self {
        ResponseState::Cancelled
    }

    pub fn value(value: TValue) -> Self {
        ResponseState::Value(value)
    }

    /// Converts a [`Result<T, E>`] into a [ResponseState] for
    /// prompts which cannot be cancelled (typically string input
    /// prompts).
    pub fn from_result(res: io::Result<TValue>) -> Self {
        match res {
            Ok(value) => Self::value(value),
            Err(err) => Self::error(err),
        }
    }

    /// Converts a [`Result<Option<T>,E>`] into a [ResponseState] with
    /// [`Ok(Some(_))`] representing success, [`Ok(None)`] representing
    /// cancellation, and [Err(_)] representing an error.
    pub fn from_result_opt(res: io::Result<Option<TValue>>) -> Self {
        match res {
            Ok(opt) => match opt {
                Some(value) => Self::value(value),
                None => Self::cancelled(),
            },
            Err(err) => Self::error(err),
        }
    }
}

/// Specialization of [ResponseState] for [String] responses.
pub type ResponseString = ResponseState<String>;
/// Specialization of [ResponseState] for [bool] responses.
pub type ResponseBool = ResponseState<bool>;
/// Specialization of [ResponseState] for [usize] or index responses.
pub type ResponseIndex = ResponseState<usize>;

impl ResponseBool {
    fn to_response_string(&self) -> ResponseString {
        match self {
            ResponseState::Value(b) => if *b {
                ResponseString::value("True".to_string())
            } else {
                ResponseString::value("False".to_string())
            },
            ResponseState::Cancelled => ResponseString::cancelled(),
            // TODO This is ugly.
            ResponseState::Error(err) => ResponseString::error(std::io::Error::new(io::ErrorKind::Other, err.to_string())),
        }
    }
}

/// Stateless struct used to group all CLI prompt functions.
pub struct Prompter;

impl Prompter {
    /// A Yes or No confirmation prompt. It can return `true` or `false` for
    /// successful answers, cancellation if the user cancels the operation,
    /// or an error for any [std::io::Error]s that occur during prompting.
    /// 
    /// ### Arguments
    /// `text` - The text to display to the user when this prompt is executed.
    pub fn confirm<S: AsRef<str>>(text: S) -> ResponseBool {
        ResponseBool::from_result_opt(
            Confirm::with_theme(&*THEME)
                .with_prompt(text.as_ref())
                .default(false)
                .interact_opt(),
        )
    }

    /// Prompt which accepts any [String] input from the user. The only validation
    /// that occurs is that the input string cannot be empty. This prompt cannot be cancelled
    /// so the only valid states returned from this function are [`ResponseState<String>::Value`]
    /// or [`ResponseState<String>::Error].
    /// 
    /// ### Arguments
    /// `text` - The text to display to the user when this prompt is executed.
    pub fn input<S: AsRef<str>>(text: S) -> ResponseString {
        ResponseString::from_result(Input::with_theme(&*THEME).with_prompt(text.as_ref()).interact_text())
    }

    /// Prompt which accepts [String] input from the user and validates that input against the
    /// given [`ValidatorFunc`]. Validator function should take a reference to the input [String]
    /// and return [Ok(())] if the input is acceptable, or an [Err(String)] describing the error.
    /// 
    /// ### Arguments
    /// `text` - The text to display to the user when this prompt is executed.
    /// `validator` - The function used to validate the input.
    pub fn validated_input<S: AsRef<str>, ValidatorFunc: FnMut(&String) -> Result<(), String>>(
        text: S,
        validator: ValidatorFunc,
    ) -> ResponseString {
        ResponseString::from_result(
            Input::with_theme(&*THEME)
                .with_prompt(text.as_ref())
                .validate_with(validator)
                .interact_text(),
        )
    }

    /// A "fuzzy select" prompt which gives the user a list of options but allows
    /// them to type and narrow down the choices. This is ideal for such inputs as
    /// already existing todos.
    pub fn fuzzy_select<TPrompt: AsRef<str>, TChoice: std::fmt::Display>(
        text: TPrompt,
        choices: &Vec<TChoice>,
    ) -> ResponseIndex {
        ResponseIndex::from_result_opt(
            FuzzySelect::with_theme(&*THEME)
                .with_prompt(text.as_ref())
                .items(&choices)
                .interact_opt(),
        )
    }

    /// A select prompt which gives the user a list of options and allows them
    /// to choose one using the arrow keys. Upon success this function returns
    /// an index pointing to the chosen response in the given `choices` array.
    pub fn select<TPrompt: Into<String>, TChoice: std::fmt::Display>(
        text: TPrompt,
        choices: &Vec<TChoice>,
    ) -> ResponseIndex {
        ResponseIndex::from_result_opt(
            Select::with_theme(&*THEME)
                .with_prompt(text)
                .items(&choices)
                .interact_opt(),
        )
    }

    pub fn for_argument(aa: &ActionArgument, existing: &Vec<&String>) -> ResponseString {
        lazy_static! {
            static ref TRUE: &'static str = "True";
            static ref FALSE: &'static str = "False";
            /// This is an example for using doc comment attributes
            static ref BOOLS: Vec<&'static str> = vec![*TRUE, *FALSE];
        }

        match aa.arg_type {
            ArgumentType::Boolean => {
                match Prompter::fuzzy_select(format!("Select value for {:?} (bool)", aa.name), &*BOOLS) {
                    ResponseState::Value(idx) => ResponseString::Value((*BOOLS[idx]).to_string().to_lowercase()),
                    ResponseState::Cancelled => ResponseString::cancelled(),
                    ResponseState::Error(err) => ResponseString::error(err),
                }
            },
            ArgumentType::String => Prompter::input(format!("Please enter value for {:?}", aa.name)),
            ArgumentType::ExistingTodo => match Prompter::fuzzy_select(format!("Please choose existing todo for {:?}", aa.name), existing) {
                ResponseState::Value(idx) => ResponseString::value(existing[idx].clone()),
                ResponseState::Cancelled => ResponseString::cancelled(),
                ResponseState::Error(err) => ResponseString::error(err),
            }
        }
    }
}
