use mindr::Config;

use std::env;
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    // TODO: improve error handling
    let user_name = env::var("USERNAME").expect("Couldn't get system user");

    let path: PathBuf = [
        "/home",
        user_name.as_str(),
        ".config",
        "mindr",
        "mindr.conf",
    ]
    .iter()
    .collect();

    path
}

// TODO: think if it's good to add other crate (not mindr) kind of like namespace for config
fn main() {
    let path = get_config_path();
    // TODO: to be used
    let config = Config::init(&path);
    config.save();
}

// -- For Future: --
// TODO: Write log messages upon each action
