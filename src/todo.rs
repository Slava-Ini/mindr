// -- Terminal Size --
// println!("{:?}", termion::terminal_size().unwrap());
// -- Cursor --
// termion::cursor::Goto(5, 10);
// -- Clear --
// print!("{}", termion::clear::All);
// println!("{}", termion::cursor::Show);
use crate::config::Action;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

use crate::config::Config;
use crate::config::Selection;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const SPACING: &'static str = "   ";

fn get_selected_str(string: &str, style: Selection) -> String {
    let selection = match style {
        Selection::Tilde => ("~", " "),
        Selection::Brackets => ("[", "]"),
        Selection::Outline | Selection::Bold => (" ", " "),
    };

    let (start_char, end_char) = selection;

    let result = string.replacen(" ", start_char, 1);
    result.replace(" ", end_char)
}

// TODO: think about creating termion struct wrapper
fn print_outline(string: &str) {
    print!(
        "{bg}{fg}{item}{bg_clear}{fg_clear}{spacing}",
        bg = termion::color::Bg(termion::color::White),
        fg = termion::color::Fg(termion::color::Black),
        item = string,
        bg_clear = termion::color::Bg(termion::color::Reset),
        fg_clear = termion::color::Fg(termion::color::Reset),
        spacing = SPACING
    );
}

fn print_bold(string: &str) {
    print!(
        "{bold}{item}{reset}{spacing}",
        bold = termion::style::Bold,
        item = string,
        reset = termion::style::Reset,
        spacing = SPACING
    );
}

fn print_item(string: &str) {
    print!("{item}{spacing}", item = string, spacing = SPACING);
}

fn prepare_print() {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
}

fn finish_print() {
    println!("");
}

#[derive(PartialEq, Clone)]
enum Menu {
    Todo,
    Done,
    Settings,
    Help,
}

impl<'a> Menu {
    fn as_str(&self) -> &'a str {
        match self {
            // TODO: Think about getting rid of spaces, instead adding them programmatically
            Menu::Todo => " TODO ",
            Menu::Done => " DONE ",
            Menu::Settings => " SETTINGS ",
            Menu::Help => " HELP ",
        }
    }
}

impl FromStr for Menu {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trim = s.trim();

        match trim {
            "TODO" => Ok(Menu::Todo),
            "DONE" => Ok(Menu::Done),
            "SETTINGS" => Ok(Menu::Settings),
            "HELP" => Ok(Menu::Help),
            _ => return Err("No such menu item!"),
        }
    }
}

#[derive(Clone)]
pub struct Todo<'a> {
    menu: [Menu; 4],
    selected_menu: Menu,
    selection_style: &'a Selection,
    key_mapping: &'a Vec<(Action, char)>,
}

impl<'a> Todo<'a> {
    pub fn init(config: &'a Config) -> Self {
        Todo {
            menu: [Menu::Todo, Menu::Done, Menu::Settings, Menu::Help],
            selected_menu: Menu::Todo,
            selection_style: &config.selection_style,
            key_mapping: &config.key_mapping,
        }
    }

    fn set_selected_menu(&mut self, menu: Menu) {
        self.selected_menu = menu;
    }

    fn get_action_char(&self, action: Action) -> char {
        let (_, key) = self
            .key_mapping
            .iter()
            .find(|(map_action, _)| *map_action == action)
            .expect("No such action exist");

        *key
    }

    fn draw_menu(&self) {
        prepare_print();

        let menu = self.menu.clone();

        let menu = menu.map(|item| {
            if item == self.selected_menu {
                return get_selected_str(item.as_str(), self.selection_style.clone());
            }

            String::from(item.as_str())
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
                        print_outline(&menu[count]);
                    }

                    if *self.selection_style == Selection::Bold {
                        print_bold(&menu[count]);
                    }
                } else {
                    print_item(&menu[count]);
                }

                count += 1;
            }
        } else {
            print!("{}", menu.join(SPACING));
        }

        finish_print();
    }

    pub fn run(&mut self) {
        print!("{}", termion::cursor::Hide);

        let stdin = stdin();
        // TODO: think about `::`
        let mut screen = termion::screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());

        self.draw_menu();

        // TODO: improve unwraps

        // TODO: think about all the clones
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char(ch) if ch == self.get_action_char(Action::Quit) => {
                    break;
                }
                // TODO: refactor
                Key::Char(ch) if ch == self.get_action_char(Action::PrevMenu) => {
                    let index = self
                        .menu
                        .iter()
                        .position(|item| *item == self.selected_menu)
                        .unwrap();
                    // TODO: probably create Menu struct to implement methods 
                    let mut chosen_menu = Menu::Todo;
                    if index > 0 {
                        chosen_menu = self.menu[index - 1].clone();
                    }

                    self.set_selected_menu(chosen_menu);
                    self.draw_menu();
                }
                Key::Char(ch) if ch == self.get_action_char(Action::NextMenu) => {
                    let index = self
                        .menu
                        .iter()
                        .position(|item| *item == self.selected_menu)
                        .unwrap();
                    let mut chosen_menu = Menu::Help;

                    if index < 3 {
                        chosen_menu = self.menu[index + 1].clone();
                    }

                    self.set_selected_menu(chosen_menu);
                    self.draw_menu();
                }
                _ => {}
            }

            screen.flush().unwrap();
        }

        print!("{}", termion::cursor::Show);
    }
}
