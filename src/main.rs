use std::env;
use std::path::PathBuf;

use mindr::app::App;
use mindr::config::Config;

// Maybe it should be inside of config and app
fn get_config_path() -> PathBuf {
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

// TODO: figure out duplication
fn get_app_path() -> PathBuf {
    let user_name = env::var("USERNAME").expect("Couldn't get system user");

    let path: PathBuf = ["/home", user_name.as_str(), ".config", "mindr", "todo.txt"]
        .iter()
        .collect();

    path
}

// TODO: think if it's good to add other crate (not mindr) kind of like namespace for config
fn main() {
    let config_path = get_config_path();
    let app_path = get_app_path();

    let config = Config::init(&config_path);
    let mut app = App::init(&config, &app_path);

    app.run();
}

// -- For Future: --
// TODO: Write log messages upon each action
