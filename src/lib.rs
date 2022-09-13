use configparser::ini::Ini;

use std::fs::{self, File};
use std::path::Path;
use std::process;
use std::slice::Iter;
use std::str::FromStr;

fn read_ini(path: &Path) -> Result<Config, String> {
    let mut ini_config = Ini::new();

    ini_config
        .load(path)
        .expect("Couldn't parse configuration file");

    let err_msg = "Couldn't get value from configuration. Please check mindr.conf";

    let auto_hide_menu = ini_config
        .getbool("general", "auto_hide_menu")?
        .expect(&err_msg);
    let display_todays = ini_config
        .getbool("general", "display_todays")?
        .expect(&err_msg);
    let remind_unfinished = ini_config
        .getbool("general", "remind_unfinished")?
        .expect(&err_msg);
    let hide_menu_timeout = ini_config
        .getuint("general", "hide_menu_timeout")?
        .expect(&err_msg) as u16;
    let selection_style = ini_config.get("style", "selection_style").expect(&err_msg);
    // TODO: change following after `from_str` error handling was improved
    let selection_style = Selection::from_str(&selection_style).unwrap();

    let mut key_mapping: Vec<(Action, String)> = vec![];

    for (index, action) in Action::iterate().enumerate() {
        let value = ini_config
            .get("key_mapping", action.as_str())
            .expect(&err_msg);
        let mapping = (action.clone(), value);

        key_mapping.insert(index, mapping);
    }

    Ok(Config {
        auto_hide_menu,
        display_todays,
        remind_unfinished,
        hide_menu_timeout,
        selection_style,
        key_mapping,
    })
}

fn write_ini(config: &Config, path: &Path) {
    let mut ini_config = Ini::new();

    ini_config.set(
        "general",
        "display_todays",
        Some(config.display_todays.to_string()),
    );
    ini_config.set(
        "general",
        "remind_unfinished",
        Some(config.remind_unfinished.to_string()),
    );
    ini_config.set(
        "general",
        "auto_hide_menu",
        Some(config.auto_hide_menu.to_string()),
    );
    ini_config.set(
        "general",
        "hide_menu_timeout",
        Some(config.hide_menu_timeout.to_string()),
    );
    ini_config.setstr(
        "style",
        "selection_style",
        Some(config.selection_style.as_str()),
    );

    for (action, key) in &config.key_mapping {
        ini_config.setstr("key_mapping", action.as_str(), Some(key));
    }

    match ini_config.write(path) {
        Err(error) => eprint!("Couldn't save configuration: {error}"),
        _ => (),
    };
}

#[derive(Debug, PartialEq)]
pub enum Selection {
    Brackets,
    Tilde,
    Outline,
}

impl Selection {
    fn as_str(&self) -> &'static str {
        match self {
            Selection::Brackets => "brackets",
            Selection::Tilde => "tilde",
            Selection::Outline => "outline",
        }
    }
}

impl FromStr for Selection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "brackets" => Ok(Selection::Brackets),
            "tilde" => Ok(Selection::Tilde),
            "outline" => Ok(Selection::Outline),
            // TODO: better error in case user changes value manually
            _ => Err(()),
        }
    }
}

// TODO: remove unused derivatives
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Action {
    Up,
    Down,
    PrevMenu,
    NextMenu,
    Mark,
}

impl Action {
    fn as_str(&self) -> &'static str {
        match self {
            Action::Up => "up",
            Action::Down => "down",
            Action::PrevMenu => "prev_menu",
            Action::NextMenu => "next_menu",
            Action::Mark => "mark",
        }
    }

    fn iterate() -> Iter<'static, Action> {
        static ACTIONS: [Action; 5] = [
            Action::Up,
            Action::Down,
            Action::PrevMenu,
            Action::NextMenu,
            Action::Mark,
        ];
        ACTIONS.iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct Config {
    // TODO: consider this one to prevent passing it to ini_write
    // pub path: &'static Path,
    pub display_todays: bool,
    pub remind_unfinished: bool,
    pub auto_hide_menu: bool,
    pub hide_menu_timeout: u16,
    pub selection_style: Selection,
    pub key_mapping: Vec<(Action, String)>,
}

impl Config {
    // TODO: make better result type
    pub fn init(path: &Path) -> Result<Config, String> {
        if !path.exists() {
            let prefix = path.parent().expect("Couldn't get the path prefix");

            fs::create_dir_all(prefix).expect("Couldn't create a directory");

            File::create(path).expect("Couldn't create configuration file");

            let config = Config {
                ..Default::default()
            };

            config.save(path);

            return Ok(config);
        }

        let config = read_ini(&path).unwrap_or_else(|err| {
            eprintln!("Couldn't read configuration file: {err}");
            process::exit(1);
        });

        Ok(config)
    }

    // TODO: make Result return for `save`
    pub fn save(&self, path: &Path) {
        write_ini(&self, path);
    }
}

impl Default for Config {
    fn default() -> Self {
        let key_mapping = vec![
            (Action::Up, String::from("j")),
            (Action::Down, String::from("k")),
            (Action::PrevMenu, String::from("h")),
            (Action::NextMenu, String::from("l")),
            (Action::Mark, String::from("Enter")),
        ];

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
