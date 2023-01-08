use crate::app::helper::{Cursor, Screen};
use crate::app::selection::PrintStyle;
use std::str::FromStr;

use crate::app::selection::Selection;
use crate::app::Action;

use termion;
use termion::event::Key;


const MENU_SPACING: &'static str = "   ";
const WRAPPER: &'static str = " ";

#[derive(PartialEq, Clone)]
pub enum MenuItem {
    Todo,
    Done,
    Settings,
    Help,
}

impl<'a> MenuItem {
    fn as_str(&self) -> String {
        match self {
            // TODO: probably move this logic to `selection` as `get_wrapped_str`
            MenuItem::Todo => format!("{WRAPPER}TODO{WRAPPER}"),
            MenuItem::Done => format!("{WRAPPER}DONE{WRAPPER}"),
            MenuItem::Settings => format!("{WRAPPER}SETTINGS{WRAPPER}"),
            MenuItem::Help => format!("{WRAPPER}HELP{WRAPPER}"),
        }
    }
}

impl FromStr for MenuItem {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trim = s.trim();

        match trim {
            "TODO" => Ok(MenuItem::Todo),
            "DONE" => Ok(MenuItem::Done),
            "SETTINGS" => Ok(MenuItem::Settings),
            "HELP" => Ok(MenuItem::Help),
            _ => return Err("No such menu item!"),
        }
    }
}

#[derive(Clone)]
pub struct Menu<'a> {
    menu: [MenuItem; 4],
    pub selected_menu: MenuItem,
    selection_style: &'a Selection,
    key_mapping: &'a Vec<(Action, char)>,
}

impl<'a> Menu<'a> {
    pub fn init(selection_style: &'a Selection, key_mapping: &'a Vec<(Action, char)>) -> Self {
        Menu {
            menu: [
                MenuItem::Todo,
                MenuItem::Done,
                MenuItem::Settings,
                MenuItem::Help,
            ],
            selected_menu: MenuItem::Todo,
            selection_style,
            key_mapping,
        }
    }

    pub fn set_selected_menu(&mut self, menu_item: MenuItem) {
        self.selected_menu = menu_item;
    }

    fn get_prev_menu(&self) -> MenuItem {
        let index = self
            .menu
            .iter()
            .position(|item| *item == self.selected_menu)
            .expect("No such menu item");

        let chosen_menu = if index > 0 {
            self.menu[index - 1].clone()
        } else {
            self.selected_menu.clone()
        };

        chosen_menu
    }

    fn get_next_menu(&self) -> MenuItem {
        let index = self
            .menu
            .iter()
            .position(|item| *item == self.selected_menu)
            .expect("No such menu item");

        let chosen_menu = if index < self.menu.len() - 1 {
            self.menu[index + 1].clone()
        } else {
            self.selected_menu.clone()
        };

        chosen_menu
    }

    pub fn render(&self) {
        Cursor::reset();

        let menu = self.menu.clone();

        for item in menu {
            let selection = if item == self.selected_menu {
                Some(self.selection_style)
            } else {
                None
            };

            let print_style = PrintStyle {
                selection,
                strikethrough: false,
                spacing: Some(MENU_SPACING),
            };

            Selection::print_styled(&item.as_str(), print_style);
        }
    }

    pub fn listen_keys(&mut self, key: &Key) {
        match key {
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::PrevMenu) => {
                let chosen_menu = self.get_prev_menu();

                self.set_selected_menu(chosen_menu);
                Screen::clear();
            }
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::NextMenu) => {
                let chosen_menu = self.get_next_menu();

                self.set_selected_menu(chosen_menu);
                Screen::clear();
            }
            _ => {}
        }
    }
}
