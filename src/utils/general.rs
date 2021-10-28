use std::str::FromStr;

pub fn s<S: Into<String>>(s: S) -> String {
    s.into()
}

pub fn string_to_bool<S: AsRef<str>>(s: S) -> Option<bool> {
    match bool::from_str(s.as_ref()) {
        Ok(b) => Some(b),
        Err(_) => match s.as_ref().to_lowercase().as_str() {
            "t" | "true" | "y" | "yes" => Some(true),
            "f" | "false" | "n" | "no" => Some(false),
            _ => None,
        },
    }
}
