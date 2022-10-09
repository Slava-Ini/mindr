use std::env;
use std::path::PathBuf;

use mindr::config::Config;
use mindr::todo::list::List;
use mindr::todo::Todo;

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

// TODO: think if it's good to add other crate (not mindr) kind of like namespace for config
fn main() {
    let path = get_config_path();
    let config = Config::init(&path);
    config.save();

    let list = List::read();
    println!("{:#?}", list);

    let mut todo = Todo::init(&config);
    todo.run();
}

// -- For Future: --
// TODO: Write log messages upon each action
