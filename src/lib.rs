use configparser::ini::Ini;

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::slice::Iter;
use std::str::FromStr;

// TODO: improve error handling
// TODO: think about where this function should go
fn load(config: &Ini) -> Config {
    // TODO: use section enum here as well
    let auto_hide_menu = config
        .getbool("general", "auto_hide_menu")
        .expect("The value is empty")
        .unwrap();
    let display_todays = config
        .getbool("general", "display_todays")
        .expect("The value is empty")
        .unwrap();
    let remind_unfinished = config
        .getbool("general", "remind_unfinished")
        .expect("The value is empty")
        .unwrap();
    let hide_menu_timeout = config
        .getuint("general", "hide_menu_timeout")
        .expect("The value is empty")
        // TODO: better message in case user changed to invalid values probably panic
        // as well as for the other non valid types in here
        .unwrap() as u16;

    let selection_style = Selection::from_str(
        config
            .get("style", "selection_style")
            // TODO: read about as str
            .expect("The value is empty")
            .as_str(),
    )
    .unwrap();

    let mut key_mapping: HashMap<Action, String> = HashMap::new();

    for mapping in Action::iterate() {
        // TODO: probably make section without spaces
        let value = config.get("key mapping", mapping.as_str()).unwrap();
        key_mapping.insert(Action::from_str(mapping.as_str()).unwrap(), value);
    }

    Config {
        auto_hide_menu,
        display_todays,
        remind_unfinished,
        hide_menu_timeout,
        selection_style,
        key_mapping,
    }
}

fn init_config(path: &Path) -> Ini {
    let mut config = Ini::new();

    config
        .load(path)
        .expect("Couldn't parse configuration file");
    config
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

#[derive(PartialEq, Eq, Hash, Debug)]
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

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Action::Up),
            "down" => Ok(Action::Down),
            "prev_menu" => Ok(Action::PrevMenu),
            "next_menu" => Ok(Action::NextMenu),
            "mark" => Ok(Action::Mark),
            // TODO: better error in case user changes value manually
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
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

        let ini_config = init_config(path);
        let config = load(&ini_config);

        Ok(config)
    }

    // TODO: probably should be outside function
    // TODO: probably path should be it's part or used only on init
    pub fn save(&self, path: &Path) {
        let mut ini_config = Ini::new();

        // TODO: refactor
        // 1. Sections as enum or struct
        // 2. iterate over enum
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
        ini_config.setstr(
            "style",
            "selection_style",
            Some(self.selection_style.as_str()),
        );

        for (action, key) in &self.key_mapping {
            ini_config.setstr("key mapping", action.as_str(), Some(key.as_str()));
        }

        ini_config.write(path).expect("Couldn't save cofiguration");
    }
}

impl Default for Config {
    fn default() -> Self {
        // TODO: probably try with vector or array of tuples
        let keys = vec![
            Action::Up,
            Action::Down,
            Action::PrevMenu,
            Action::NextMenu,
            Action::Mark,
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
