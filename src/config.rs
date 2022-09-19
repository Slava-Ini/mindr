use configparser::ini;

use std::fs::{self, File};
use std::path::Path;
use std::process;
use std::slice::Iter;
use std::str::FromStr;

// For the future:
// TODO: Go through methods and decide which should be public
// TODO: Make stronger typing for key values rather than just String
// For now any key value will do, even empty, maybe I can try to store key codes
// Or at leas make a check that key value is non-empty
// TODO: Sometimes upon config save changes it's sections placing, not vital but it would be nice
// to have it always be same + add newline between sections (check lib's version for pretty write)
// TODO: `read_ini` and `write_ini` could be improved with iterators but it takes some research
// on how to implement access struct fields using string and get it's fields as string without
// writing to much code

fn read_ini(path: &Path) -> Result<Config, String> {
    let mut ini_config = ini::Ini::new();

    ini_config
        .load(path)
        .expect("Couldn't parse configuration file");

    let auto_hide_menu = ini_config
        .getbool("general", "auto_hide_menu")?
        .unwrap_or_else(|| {
           eprintln!("Couldn't get 'auto_hide_menu' value: Not a boolean. 'auto_hide_menu' will be set to default 'false'");
           false
        });

    let display_todays = ini_config
        .getbool("general", "display_todays")?
        .unwrap_or_else(|| {
           eprintln!("Couldn't get 'display_todays' value: Not a boolean. 'display_todays' will be set to default 'true'");
           true
        });

    let remind_unfinished = ini_config
        .getbool("general", "remind_unfinished")?
        .unwrap_or_else(|| {
           eprintln!("Couldn't get 'remind_unfinished' value: Not a boolean. 'remind_unfinished' will be set to default 'false'");
           true
        });

    let hide_menu_timeout = ini_config
        .getuint("general", "hide_menu_timeout")?
        .unwrap_or_else(|| {
           eprintln!("Couldn't get 'remind_unfinished' value: Not a valid number. 'remind_unfinished' will be set to default '500'");
           500
        });
    let hide_menu_timeout = match hide_menu_timeout {
        n if n > 60000 => {
            eprintln!("The value of 'hide_menu_timeout' can not be greater than 60000, 'hide_menu_timeout' will be set to default '500'");
            500 as u16
        }
        _ => hide_menu_timeout as u16,
    };

    let selection_style = ini_config
        .get("style", "selection_style")
        .unwrap_or_else(|| {
            eprintln!("Couldn't get 'selection_style' value: Not a string. 'selection_style' will be set to default 'Brackets'");
            String::from("brackets")
        });
    let selection_style = Selection::from_str(&selection_style).unwrap_or_else(|err| {
        eprintln!("Couldn't get 'selection_style' style: {err}.");
        Selection::Brackets
    });

    let mut key_mapping: Vec<(Action, String)> = vec![];

    for (index, action) in Action::iterate().enumerate() {
        let key = ini_config
            .get("key_mapping", action.as_str())
            .unwrap_or_else(|| {
                let default_config = Config::default();

                let (_, default_key): &(Action, String) = default_config
                    .key_mapping
                    .iter()
                    .find(|(default_action, _)| default_action.as_str() == action.as_str())
                    .unwrap();

                eprintln!(
                    "Couldn't get '{:?}' action key. '{:?}' will be set to default {:?}",
                    action, action, &default_key
                );

                String::from(default_key.as_str())
            });

        let mapping = (action.clone(), key);

        key_mapping.insert(index, mapping);
    }

    Ok(Config {
        auto_hide_menu,
        display_todays,
        remind_unfinished,
        hide_menu_timeout,
        selection_style,
        key_mapping,
        path: &path,
    })
}

fn write_ini(config: &Config, path: &Path) {
    let mut ini_config = ini::Ini::new();

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
        Err(error) => {
            panic!("Couldn't save configuration: {error}");
        }
        _ => (),
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection {
    Brackets,
    Tilde,
    Outline,
    Bold,
}

impl Selection {
    fn as_str(&self) -> &'static str {
        match self {
            Selection::Brackets => "brackets",
            Selection::Tilde => "tilde",
            Selection::Outline => "outline",
            Selection::Bold => "bold",
        }
    }
}

impl FromStr for Selection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "brackets" => Ok(Selection::Brackets),
            "tilde" => Ok(Selection::Tilde),
            "outline" => Ok(Selection::Outline),
            "bold" => Ok(Selection::Bold),
            _ => {
                return Err(
                    "No such selection style available, try using 'brackets/tilde/outline/bold'",
                )
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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
pub struct Config<'a> {
    pub path: &'a Path,
    pub display_todays: bool,
    pub remind_unfinished: bool,
    pub auto_hide_menu: bool,
    pub hide_menu_timeout: u16,
    pub selection_style: Selection,
    pub key_mapping: Vec<(Action, String)>,
}

impl<'a> Config<'a> {
    pub fn init(path: &'a Path) -> Self {
        if !path.exists() {
            let prefix = path.parent().expect("Couldn't get the path prefix");

            fs::create_dir_all(prefix).expect("Couldn't create a directory");
            File::create(path).expect("Couldn't create configuration file");

            let mut config = Config::default();

            config.set_path(path);
            config.save();

            return config;
        }

        let config = read_ini(&path).unwrap_or_else(|err| {
            eprintln!("Couldn't read configuration file: {err}");
            process::exit(1);
        });

        config
    }

    fn set_path(&mut self, path: &'a Path) {
        self.path = path;
    }

    pub fn save(&self) {
        write_ini(&self, self.path);
    }
}

impl<'a> Default for Config<'a> {
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
            path: Path::new(""),
        }
    }
}
