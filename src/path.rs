use log::{info, warn};
use std::path;

pub fn exists(path_name: &str) -> bool {
    match path::Path::new(path_name).exists() {
        true => {
            info!("Exists: {}", path_name);
            true
        }
        false => {
            warn!("Does *not* exist: {}", path_name);
            false
        }
    }
}

pub fn exists_if(path_name: &str, condition: bool) -> bool {
    if condition {
        exists(path_name)
    } else {
        warn!("Does *not* exist: {}", path_name);
        false
    }
}
