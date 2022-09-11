use configparser::ini::Ini;

use std::collections::HashMap;
use std::env;
use std::fmt::{write, Display};
use std::fs::{self, File};
use std::ops::Index;
use std::path::Path;

// TODO: probably needs refactor to put it somewhere else
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

fn capitalize(s: &str) -> String {
    format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

#[derive(Debug)]
pub enum Selection {
    Brackets(String),
    Tilde(String),
    Outline(String),
}

impl Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Up(String),
    Down(String),
    PrevMenu(String),
    NextMenu(String),
    Mark(String),
}

#[derive(Debug)]
pub struct Config {
    pub display_todays: bool,
    pub remind_unfinished: bool,
    pub auto_hide_menu: bool,
    pub hide_menu_timeout: u16,
    pub selection_style: Selection,
    pub key_mapping: HashMap<Action, String>,
}

impl Config {
    // TODO: make better result type
    pub fn init(&self) -> Result<&Config, String> {
        let config_path = get_config_path();
        let path = Path::new(&config_path);

        if !path.exists() {
            let prefix = path.parent().expect("Couldn't get the path prefix");

            fs::create_dir_all(prefix).expect("Couldn't create a directory");
            File::create(path).expect("Couldn't create configuration file");

            let config = Config {
                ..Default::default()
            };

            config.save(path);
        }

        let config = init_config(path);
        Ok(self)
    }

    fn save(self, path: &Path) {
        let mut ini_config = Ini::new();

        // let generals = vec!["display_todays", "remind_unfinished", "auto_hide_menu"];

        // for item in generals {
        //     ini_config.set("general", item, Some(self[item].to_string()));
        // }
        ini_config.set(
            "general",
            "display_todays",
            Some(self.display_todays.to_string()),
        );
        ini_config.set(
            "general",
            "remind_unfinished",
            Some(self.remind_unfinished.to_string()),
        );
        ini_config.set(
            "general",
            "auto_hide_menu",
            Some(self.auto_hide_menu.to_string()),
        );
        ini_config.set(
            "general",
            "hide_menu_timeout",
            Some(self.hide_menu_timeout.to_string()),
        );
        ini_config.set(
            "style",
            "selection_style",
            Some(self.selection_style.to_string()),
        );

        ini_config.write(path).expect("Couldn't save cofiguration");
    }
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
            selection_style: Selection::Brackets(String::from("brackets")),
            key_mapping,
        }
    }
}

impl Index<&'_ str> for Config {
    type Output = bool;
    fn index(&self, index: &'_ str) -> &Self::Output {
        match index {
            "display_todays" => &self.display_todays,
            "remind_unfinished" => &self.remind_unfinished,
            "auto_hide_menu" => &self.auto_hide_menu,
            // "hide_menu_timeout" => &self.hide_menu_timeout,
            _ => panic!("Unknown field: {}", index),
        }
    }
}
