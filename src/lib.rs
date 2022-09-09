use configparser::ini::Ini;

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::path::Path;

enum Selection {
    Brackets,
    Tilde,
    Outline,
}

#[derive(PartialEq, Eq, Hash)]
enum Action {
    Up(String),
    Down(String),
    PrevMenu(String),
    NextMenu(String),
    Mark(String),
}

struct Config {
    display_todays: bool,
    remind_unfinished: bool,
    auto_hide_menu: bool,
    hide_menu_timeout: u16,
    selection_style: Selection,
    key_mapping: HashMap<Action, String>,
}

impl Default for Config {
    fn default() -> Self {
        let keys = vec![
            Action::Up(String::from("up")),
            Action::Down(String::from("down")),
            Action::PrevMenu(String::from("prev_menu")),
            Action::NextMenu(String::from("next_menu")),
            Action::Mark(String::from("mark_done")),
        ];

        let actions = vec![
            String::from("j"),
            String::from("k"),
            String::from("h"),
            String::from("l"),
            String::from("Enter"),
        ];

        let key_mapping: HashMap<Action, String> =
            keys.into_iter().zip(actions.into_iter()).collect();

        Config {
            display_todays: true,
            remind_unfinished: true,
            auto_hide_menu: false,
            hide_menu_timeout: 500,
            selection_style: Selection::Brackets,
            key_mapping,
        }
    }
}

// TODO: create actual use case
fn example_use() {
    let config = Config {
        ..Default::default()
    };
}

fn get_config_path() -> String {
    let user_name = env::var("USERNAME")
        .expect("Couldn't get system user")
        .to_owned();

    format!("/home/{user_name}/.config/mindr/mindr.conf")
}

fn init_config(path: &Path) -> Ini {
    let mut config = Ini::new();

    config
        .load(path)
        .expect("Couldn't parse configuration file");
    config
}

pub fn get_configuration() -> Ini {
    let config_path = get_config_path();
    let path = Path::new(&config_path);

    if !path.exists() {
        let prefix = path.parent().expect("Couldn't get the path prefix");

        fs::create_dir_all(prefix).expect("Couldn't create a directory");
        File::create(path).expect("Couldn't create configuration file");
    }

    let config = init_config(path);

    config
}
