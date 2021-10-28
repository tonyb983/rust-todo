use itertools::Itertools;
use owo_colors::{colors, OwoColorize};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    str::FromStr,
    time::{Duration, Instant},
};

use super::command_error::CommandError;
use crate::{
    input::prompter::{Prompter, ResponseBool},
    state::actions::action_payload::ActionPayload,
    utils::{
        cereal::{Cereal, EncodingType},
        fs::FileSystem,
        general::s,
    },
};

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub enum DiffEntry {
    TodoNotFound {
        todo: String,
        this_has: bool,
        that_has: bool,
    },
    TodoStatusMistake {
        todo: String,
        this_status: bool,
        that_status: bool,
    },
}

impl DiffEntry {
    pub fn to_string(&self) -> String {
        match self {
            DiffEntry::TodoNotFound {
                todo,
                this_has,
                that_has,
            } => format!(
                "Todo {:?} is in {} but not {}.",
                todo,
                if *that_has { "that" } else { "this" },
                if *this_has { "that" } else { "this" }
            ),
            DiffEntry::TodoStatusMistake {
                todo,
                this_status,
                that_status,
            } => format!(
                "Todo {:?} is marked as {}complete in this but {}complete in that.",
                todo,
                if *this_status { "" } else { "in" },
                if *that_status { "" } else { "in" }
            ),
        }
    }
}

impl std::fmt::Display for DiffEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize, Clone)]
pub enum DiffResult {
    Same,
    Changes(Vec<DiffEntry>),
}

pub const DEFAULT_ENCODING: EncodingType = EncodingType::MsgPack;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TodoList {
    map: HashMap<String, bool>,
}

impl TodoList {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn count(&self) -> usize {
        self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn any_with_status(&self, status: bool) -> bool {
        if self.is_empty() {
            return false;
        }

        for (k, v) in self.map.iter() {
            if *v == status {
                return true;
            }
        }

        return false;
    }

    fn create_backup(&self) -> Result<(), std::io::Error> {
        Ok(())
    }

    /// TODO Refactor to use [`crate::utils::fs::FileSystem`]
    pub fn save_to_disk(&self) -> Result<(), String> {
        Cereal::serialize_with(DEFAULT_ENCODING, &self).map_or_else(
            |err| Err(err),
            |bytes| {
                FileSystem::save_bytes(format!("data.{}", DEFAULT_ENCODING.get_file_ext()), &bytes)
                    .map_err(|io_err| io_err.to_string())
            },
        )

        // let mut content = String::new();
        // for (k, v) in &self.map {
        //     let record = format!("{}\t{}\n", k, v);
        //     content.push_str(&record);
        // }

        // std::fs::write("db.txt", content)
    }

    /// TODO Refactor to use [`crate::utils::fs::FileSystem`]
    pub fn load_from_disk() -> Result<TodoList, String> {
        let file_name = format!("data.{}", DEFAULT_ENCODING);
        let path = std::path::Path::new(&file_name);
        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File {:?} not found!", file_name),
            )
            .to_string());
        }

        FileSystem::load_bytes(path).map_or_else(
            |io_err| Err(io_err.to_string()),
            |bytes| Cereal::deserialize_with(DEFAULT_ENCODING, &bytes),
        )
    }

    pub fn get_todos_text(&self) -> Vec<&String> {
        self.map.keys().collect()
    }

    pub fn get_todos(&self) -> Vec<(&String, &bool)> {
        if self.is_empty() {
            return Vec::new();
        }

        self.map.iter().map(|(k, v)| (k, v)).collect_vec()
    }

    pub fn get_todos_with_status(&self, status: bool) -> Vec<&String> {
        self.map
            .iter()
            .filter(|kv| *kv.1 == status)
            .map(|(k, v)| k)
            .collect_vec()
    }

    pub fn for_each_todo<Action: Fn(&(&String, &bool))>(&self, action: Action) {
        self.map.iter().for_each(|(kv)| action(&kv))
    }

    pub fn map_todos<Output, Func: Fn((&String, &bool)) -> Output>(
        &self,
        func: Func,
    ) -> Vec<Output> {
        // self.map.iter().map(func).collect_vec()
        let mut output = Vec::new();
        for (kv) in self.map.iter() {
            output.push(func(kv));
        }

        return output;
    }

    pub fn filter_todos<Pred: Fn(&(&String, &bool)) -> bool>(&self, pred: Pred) -> Vec<&String> {
        self.map.iter().filter(pred).map(|(k, v)| k).collect_vec()
    }

    pub fn filter_map_todos<Output, Pred, Func>(&self, pred: Pred, func: Func) -> Vec<Output>
    where
        Pred: Fn(&(&String, &bool)) -> bool,
        Func: Fn(&(&String, &bool)) -> Output,
    {
        self.map
            .iter()
            .filter(pred)
            .map(|(kv)| func(&kv))
            .collect_vec()
    }

    pub fn clone(&self) -> Self {
        let mut new_map = self.map.clone();
        Self { map: new_map }
    }

    pub fn diff_with(&self, other: &Self) -> DiffResult {
        if self.is_empty() && other.is_empty() {
            return DiffResult::Same;
        }

        let mut this_copy = self.map.clone();
        let mut that_copy = other.map.clone();
        let mut changes: Vec<DiffEntry> = Vec::new();

        for (this_todo, this_status) in self.map.iter() {
            match that_copy.remove_entry(this_todo) {
                Some((that_todo, that_status)) => {
                    if that_status != *this_status {
                        changes.push(DiffEntry::TodoStatusMistake {
                            todo: (*this_todo).clone(),
                            this_status: *this_status,
                            that_status,
                        });
                    }
                }
                None => {
                    changes.push(DiffEntry::TodoNotFound {
                        this_has: true,
                        that_has: false,
                        todo: (*this_todo).clone(),
                    });
                }
            }
            this_copy.remove(this_todo);
        }

        assert_eq!(
            this_copy.len(),
            0,
            "After iterating through self.map, this_copy should be empty. this_copy = {:?}",
            this_copy
        );

        for (key, value) in that_copy.iter() {
            changes.push(DiffEntry::TodoNotFound {
                todo: (*key).clone(),
                this_has: false,
                that_has: true,
            })
        }

        if changes.is_empty() {
            DiffResult::Same
        } else {
            DiffResult::Changes(changes)
        }
    }

    pub fn add_todo<S: AsRef<str>>(&mut self, todo: S, status: bool) -> Result<(), CommandError> {
        if todo.as_ref().is_empty() {
            return Err(CommandError::InputInvalid("Todo is empty".to_string()));
        }

        if self.map.contains_key(todo.as_ref()) {
            return Err(CommandError::TodoAlreadyExists);
        }

        self.map.insert(todo.as_ref().to_string(), status);
        Ok(())
    }

    pub fn remove_todo<Text: AsRef<str>>(&mut self, todo: Text) -> Option<(String, bool)> {
        self.map.remove_entry(todo.as_ref())
    }

    pub fn clear_todos(&mut self) {
        self.map.clear()
    }

    /// TODO Need to clean this up. Figure out whether this function wants to interact with the
    ///     user or whether it wants to execute commands (i.e. it should not be doing both).
    pub fn apply_action(&mut self, action: ActionPayload) -> Result<(), CommandError> {
        match action {
            ActionPayload::Add(key) => {
                return self.add_todo(key, false);
            }
            ActionPayload::Clear => match Prompter::confirm("Are you sure?") {
                ResponseBool::Value(value) => {
                    if value {
                        println!("Clearing all todos...");
                        self.clear_todos();
                        println!("Todos cleared.");
                    } else {
                        println!("Cancelling clear operation.");
                    }
                }
                ResponseBool::Cancelled => {
                    println!("Cancelling clear operation.");
                }
                ResponseBool::Error(err) => {
                    println!("Error during prompt: {:?}", err);
                }
            },
            ActionPayload::Edit(existing, new_text) => {
                if let Some(status) = self.map.remove(&existing) {
                    self.map.insert(new_text.to_string(), status);
                } else {
                    return Err(CommandError::TodoNotFound);
                }
            },
            ActionPayload::List => {
                if self.is_empty() {
                    println!("No todos in database, you're either very on top of things or slacking reallllllly bad.");
                    return Ok(());
                }

                println!();
                println!("All Todos\n--- -----");
                self.for_each_todo(|(kv)| {
                    println!("{} {:?}", if *kv.1 { "[X]" } else { "[ ]" }, kv.0)
                });
                println!();
                return Ok(());
            }
            ActionPayload::ListWithStatus(kind) => {
                // TODO This might have performance implications for very large data-sets, keep an eye out.
                if !self.any_with_status(kind) {
                    println!(
                        "There are no {} todos in the database.",
                        if kind { "completed" } else { "incomplete" }
                    );
                    return Ok(());
                }

                println!(
                    "{} Todos\n{} -----",
                    if kind { "Completed" } else { "Incomplete" },
                    if kind { "---------" } else { "----------" }
                );
                for (k, v) in self.map.iter().filter(|(kv)| *kv.1 == kind) {
                    println!("\t* {:?}", *k);
                }
            }
            ActionPayload::Remove(key) => {
                if key.is_empty() {
                    return Err(CommandError::InputInvalid(s("Todo is empty")));
                }

                if let Some(_) = self.remove_todo(&key) {
                    return Ok(());
                } else {
                    return Err(CommandError::TodoNotFound);
                }
            }
            ActionPayload::Set(key, val) => {
                if key.is_empty() {
                    return Err(CommandError::InputInvalid(s("Todo is empty")));
                }

                self.map.insert(key, val);
            }
            ActionPayload::Other(input) => {
                return self.run_debug_command(input);
            }
        }

        Ok(())
    }
}

/// Debug command functions.
impl TodoList {
    fn run_debug_command<S: AsRef<str>>(&self, input: S) -> Result<(), CommandError> {
        match input.as_ref().to_lowercase().as_str() {
            "encoding" => {
                return self.run_encoding_test();
            }
            "diff" => {
                return self.run_diff_test();
            }
            _ => {
                println!("Unknown debug command {:?}", input.as_ref());
            }
        }
        Ok(())
    }

    fn run_encoding_test(&self) -> Result<(), CommandError> {
        struct RunTime {
            se_duration: Duration,
            de_duration: Duration,
        }

        impl RunTime {
            fn empty() -> Self {
                Self {
                    se_duration: Duration::default(),
                    de_duration: Duration::default(),
                }
            }

            fn from_se_time(d: Duration) -> Self {
                Self {
                    se_duration: d,
                    de_duration: Duration::default(),
                }
            }

            fn set_se_duration(&mut self, d: Duration) {
                self.se_duration = d;
            }

            fn set_de_duration(&mut self, d: Duration) {
                self.de_duration = d;
            }
        }

        println!("Running serialization comparison...\n");
        let mut byte_map: HashMap<EncodingType, Vec<u8>> = HashMap::new();
        let mut time_map: HashMap<EncodingType, RunTime> = HashMap::new();

        for ty in EncodingType::all() {
            println!("Starting {}", ty);
            let start = Instant::now();
            match Cereal::serialize_with(ty, self) {
                Ok(data) => {
                    let duration = start.elapsed();
                    time_map.insert(ty, RunTime::from_se_time(duration));
                    byte_map.insert(ty, data);
                }
                Err(err) => {
                    println!("There was an error with {:?}: {}", ty, err);
                }
            }

            println!("{} Serialization Complete.", ty);
        }

        for (ty, bytes) in byte_map.iter() {
            let file_name = format!("./data/{}.dat", ty);
            match FileSystem::save_bytes(&file_name, bytes) {
                Ok(_) => println!("Wrote {} successfully!", &file_name),
                Err(e) => println!("Error writing {}: {}", &file_name, e),
            }
        }

        for ty in EncodingType::all() {
            let file_name = format!("./data/{}.dat", ty);
            let path = std::path::Path::new(&file_name);
            match path.try_exists() {
                Ok(exists) => {
                    if !exists {
                        println!("Error, path {:?} does not exist!", path);
                        continue;
                    }

                    match FileSystem::load_bytes(path) {
                        Ok(bytes) => {
                            let start = Instant::now();
                            match Cereal::deserialize_with::<TodoList>(ty, &bytes) {
                                Ok(recreated) => {
                                    let duration = start.elapsed();
                                    println!(
                                        "Encoding {} took {}ms to deserialize.",
                                        ty,
                                        duration.as_millis()
                                    );
                                    time_map.entry(ty).and_modify(|rt| {
                                        rt.de_duration = duration;
                                    });
                                    match self.diff_with(&recreated) {
                                        DiffResult::Same => {
                                            println!("Recreated todo-list from {} data and it matches the original!", ty);
                                        }
                                        DiffResult::Changes(diffs) => {
                                            println!("Recreated todo-list {:?} from {} data and it does NOT match the original.", recreated, ty);
                                            println!(
                                                "Diff Results:\n\t{}\n",
                                                diffs
                                                    .iter()
                                                    .map(|diff_entry| diff_entry.to_string())
                                                    .join("\n\t")
                                            )
                                        }
                                    }
                                }
                                Err(e) => println!("Error deserializing todolist: {}", e),
                            }
                        }
                        Err(e) => println!("Error loading bytes from file: {}", e),
                    }
                }
                Err(err) => println!("Error with path {:?}: {}", path, err),
            }
        }

        println!("Serialization Size Results");
        println!(
            "{:^12}{:^7}",
            "Encoding".fg::<colors::White>().underline(),
            "Bytes".fg::<colors::Cyan>().underline()
        );
        let mut first = true;
        for (ty, bytes) in byte_map
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.1.len(), &b.1.len()))
        {
            if first {
                println!(
                    "{:^12}{:^7}",
                    ty.fg::<colors::White>(),
                    bytes.len().fg::<colors::BrightGreen>()
                );

                first = false
            } else {
                println!(
                    "{:^12}{:^7}",
                    ty.fg::<colors::White>(),
                    bytes.len().fg::<colors::Cyan>()
                );
            }
        }
        println!();

        println!("Serialization Time Results (in MS)");
        println!(
            "{:^12}{:^9}{:^9}",
            "Encoding".fg::<colors::White>().underline(),
            "Se Time".fg::<colors::Cyan>().underline(),
            "De Time".fg::<colors::Yellow>().underline(),
        );
        first = true;
        for (ty, rt) in time_map
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.1.se_duration, &b.1.se_duration))
        {
            if first {
                println!(
                    "{:^12}{:^9}{:^9}",
                    ty.fg::<colors::White>(),
                    format!("{:?}", rt.se_duration).fg::<colors::BrightGreen>(),
                    format!("{:?}", rt.de_duration).fg::<colors::BrightGreen>(),
                );
                first = false;
            } else {
                println!(
                    "{:^12}{:^9}{:^9}",
                    ty.fg::<colors::White>(),
                    format!("{:?}", rt.se_duration).fg::<colors::Cyan>(),
                    format!("{:?}", rt.de_duration).fg::<colors::Yellow>(),
                );
            }
        }
        println!();

        Ok(())
    }

    fn run_diff_test(&self) -> Result<(), CommandError> {
        if self.is_empty() {
            return Err(CommandError::InputInvalid(
                "TodoList must have at least 1 entry in order to run diff test!".to_string(),
            ));
        }

        let mut other = self.clone();
        println!("Diffing against cloned other...");
        match self.diff_with(&other) {
            DiffResult::Same => {
                println!("Diff returned Same!");
            }
            DiffResult::Changes(diffs) => {
                println!("Uh-oh, diff returned the following changes:");
                for (i, d) in diffs.iter().enumerate() {
                    println!("#{}: {}", i + 1, d);
                }

                println!("");
            }
        }

        let mut rng = rand::thread_rng();
        let changes: usize = rng.gen_range(1..self.len());
        println!("Making {} changes.", changes);
        for i in 0..changes {
            match rng.gen_range(0..3) {
                0 => {
                    // Change status
                    let idx = rng.gen_range(0..other.len());
                    let (todo, status) = match other.map.iter().nth(idx) {
                        Some((s, b)) => (s.as_str().to_string(), *b),
                        None => unreachable!(),
                    };
                    println!(
                        "\t- Change #{}: Changing status of {:?} from {} to {}",
                        i + 1,
                        &todo,
                        status,
                        !status
                    );
                    other.map.insert(todo, !status);
                }
                1 => {
                    // Add Todo
                    let to_add = format!("Here is a random todo I added. {}", rng.gen::<usize>());
                    let status = rng.gen_bool(0.5);
                    println!(
                        "\t- Change #{}: Adding random todo {:?} with status {}",
                        i + 1,
                        &to_add,
                        status
                    );
                    other.add_todo(&to_add, status);
                }
                2 => {
                    // Remove Todo
                    let idx = rng.gen_range(0..other.len());
                    let existing = match other.map.iter().nth(idx) {
                        Some((s, b)) => s.to_owned(),
                        None => unreachable!(),
                    };
                    println!(
                        "\t- Change #{}: Removing random todo {:?}",
                        i + 1,
                        &existing
                    );
                    other.remove_todo(&existing);
                }
                _ => unreachable!(),
            }
        }

        println!("Diffing against modified other...");
        match self.diff_with(&other) {
            DiffResult::Same => {
                println!("Uh-oh, diff returned Same!");
            }
            DiffResult::Changes(diffs) => {
                // TODO This is throwing false positives when the status is changed on an added todo.
                if diffs.len() == changes {
                    println!(
                        "Hurray, diff returned the correct number of changes ({}).",
                        changes
                    );
                } else {
                    println!(
                        "Uh oh, there are {} diff results but {} changes were made.",
                        diffs.len(),
                        changes
                    );
                }

                println!();
                println!("Diff Entries:");
                for (i, d) in diffs.iter().enumerate() {
                    println!("#{}: {}", i + 1, d);
                }

                println!("");
            }
        }

        Ok(())
    }
}
