use mindr;
use std::fs::File;

use std::env;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    // TODO: improve error handling
    let user_name = env::var("USERNAME").expect("Couldn't get system user");

    let path: PathBuf = ["/home", user_name.as_str(), ".config", "mindr", "mindr.conf"]
        .iter()
        .collect();

    path
}

// TODO: think if it's good to add other crate (not mindr) kind of like namespace for config
fn main() {
    let path = get_config_path();
    let config = mindr::Config::init(&path);
}

// --- Future ---
