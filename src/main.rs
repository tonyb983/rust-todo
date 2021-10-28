#![allow(unused)]
#![feature(path_try_exists)]

use mimalloc::MiMalloc;

mod config;
mod input;
// mod service;
mod state;
mod todos;
mod utils;

use crate::input::prompter::{Prompter, ResponseIndex, ResponseString};
use crate::{
    state::actions::action_type::ActionType,
    todos::todolist::TodoList,
};

// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    if std::env::args().len() < 2 {
        println!("No args passed, launching REPL");
        let mut todo_list = TodoList::load_from_disk().expect("Unable to load Todo-List!");
        println!("Loaded {} todos from disk.", todo_list.len());
        repl(&mut todo_list);
        if let Err(err) = todo_list.save_to_disk() {
            println!("Error saving Todo-List database! {}", err);
        }

        return;
    }

    let cmd_raw: String = std::env::args().nth(1).unwrap_or("".to_string());
    let args_raw: Vec<String> = std::env::args().skip(2).collect::<Vec<String>>();
    println!(
        "Input Command = {:?}\nInput Args = {:?}\n",
        cmd_raw, args_raw
    );

    let parse_result = ActionType::try_parse_cmd(&cmd_raw);

    if let Err(err) = &parse_result {
        println!("Error while parsing command!\n{}\n\n", err.to_string());
        return;
    }

    let action = parse_result.unwrap();

    let payload = match action.try_create_payload(&args_raw) {
        Ok(act) => act,
        Err(err) => {
            println!("Error while validating action!\n{}\n\n", err.to_string());
            return;
        }
    };

    let mut todo_list = TodoList::load_from_disk().map_or_else(|_| TodoList::new(), |tl| tl);
    println!("Loaded {} todos from disk.", todo_list.len());
    println!(
        "Loaded Todo-List containing {:?} {}.",
        todo_list.len(),
        if todo_list.len() == 1 {
            "entry"
        } else {
            "entries"
        }
    );

    if let Err(err) = todo_list.apply_action(payload) {
        println!(
            "There was an error applying command to the Todo-List: {:?}",
            err.to_string()
        );
    }

    println!(
        "Todo-List contains {:?} {}",
        todo_list.len(),
        if todo_list.len() == 1 {
            "entry"
        } else {
            "entries"
        }
    );
    println!("Writing Todo-List...");

    match todo_list.save_to_disk() {
        Ok(_) => println!("Success!"),
        Err(e) => println!("An error has occurred! {:#?}", e),
    }
}

fn repl(todo_list: &mut TodoList) {
    let actions: Vec<ActionType> = ActionType::all_actions();
    let mut choices: Vec<String> = ActionType::all_action_names();
    choices.push("Exit".to_string());
    let exit_number = choices.len() - 1;

    println!("Actions = {:#?}", actions);
    println!("Choices = {:#?}", choices);
    println!("Exit Number = {:#?}", exit_number);

    loop {
        match Prompter::select("Please choose an option", &choices) {
            ResponseIndex::Value(i) => {
                println!("i = {}", i);

                if i == exit_number {
                    break;
                }

                let action_args = actions[i].get_arguments();
                let mut args: Vec<String> = vec![];
                for at in &action_args {
                    loop {
                        match Prompter::for_argument(at, &todo_list.get_todos_text()) {
                            ResponseString::Value(s) => {
                                args.push(s.clone());
                                break;
                            },
                            ResponseString::Cancelled => {
                                println!("Argument prompt cancelled.");
                                continue;
                            },
                            ResponseString::Error(err) => {
                                println!("Error during argument prompt: {}", err);
                                continue;
                            }
                        }
                    }
                }

                match actions[i].try_create_payload(&args) {
                    Ok(payload) => match todo_list.apply_action(payload) {
                        Ok(_) => println!(""),
                        Err(err) => println!("Error applying action.\n{}\n", err.to_string()),
                    },
                    Err(err) => println!("Error creating action.\n{}\n", err.to_string()),
                };
            }
            ResponseIndex::Cancelled => {
                println!("Selection cancelled, exiting program...");
                return;
            }
            ResponseIndex::Error(err) => {
                println!("An error has occurred: {}", err);
                return;
            }
        }
    }
}
