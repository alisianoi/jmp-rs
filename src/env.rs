use log::{debug, error};
use std::env;
use std::env::VarError;

pub fn var(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(value) => {
            debug!("${} is: {}", name, value);
            Some(value)
        }
        Err(VarError::NotPresent) => {
            debug!("${} is not present", name);
            None
        }
        Err(VarError::NotUnicode(_)) => {
            let msg = format!("${} is not Unicode!", name);
            error!("${}", msg);
            panic!(msg);
        }
    }
}

pub fn var_or_default(name: &str) -> (String, bool) {
    match var(name) {
        Some(value) => (value, true),
        None => (format!("${}", name), false),
    }
}
