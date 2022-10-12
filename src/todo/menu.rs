use std::io::{Stdin, Stdout, Write};
use std::str::FromStr;

use crate::todo::helper::{finish_print, prepare_print, print_item};
use crate::todo::selection::Selection;
use crate::todo::Action;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;

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
    selected_menu: MenuItem,
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
        // prepare_print();

        let menu = self.menu.clone();

        let menu = menu.map(|item| {
            if item == self.selected_menu {
                return Selection::get_selected_str(&item.as_str(), self.selection_style.clone());
            }

            item.as_str()
        });

        if *self.selection_style == Selection::Outline || *self.selection_style == Selection::Bold {
            let index = self
                .menu
                .iter()
                .position(|item| *item == self.selected_menu)
                .unwrap();

            let mut count = 0;

            while count < self.menu.len() {
                if count == index {
                    if *self.selection_style == Selection::Outline {
                        Selection::print_outline(&menu[count], Some(MENU_SPACING));
                    }

                    if *self.selection_style == Selection::Bold {
                        Selection::print_bold(&menu[count], Some(MENU_SPACING));
                    }
                } else {
                    print_item(&menu[count], MENU_SPACING);
                }

                count += 1;
            }
        } else {
            print!("{}", menu.join(MENU_SPACING));
        }

        // finish_print();
    }

    pub fn listen_keys(&mut self, stdin: Stdin, mut screen: AlternateScreen<RawTerminal<Stdout>>) {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char(ch) if ch == Action::get_action_char(self.key_mapping, Action::Quit) => {
                    break;
                }
                Key::Char(ch)
                    if ch == Action::get_action_char(self.key_mapping, Action::PrevMenu) =>
                {
                    let chosen_menu = self.get_prev_menu();

                    self.set_selected_menu(chosen_menu);
                    self.render();
                }
                Key::Char(ch)
                    if ch == Action::get_action_char(self.key_mapping, Action::NextMenu) =>
                {
                    let chosen_menu = self.get_next_menu();

                    self.set_selected_menu(chosen_menu);
                    self.render();
                }
                _ => {}
            }

            screen.flush().unwrap();
        }
    }
}
