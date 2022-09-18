// -- Color --
// println!("{} hello", termion::color::Fg(termion::color::Blue));
// -- Terminal Size --
// println!("{:?}", termion::terminal_size().unwrap());
// -- Background --
// println!(
//     "{}Background{}",
//     termion::color::Bg(termion::color::Cyan),
//     termion::color::Bg(termion::color::Reset)
// );
// -- Style --
// println!("{} I'm bold", termion::style::Bold);
// -- Cursor --
// termion::cursor::Goto(5, 10);
// -- Clear --
// print!("{}", termion::clear::All);
// println!("{}", termion::cursor::Show);
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

use crate::config::Config;
use crate::config::Selection;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn get_selected_str(string: &str, style: Selection) -> String {
    let selection = match style {
        Selection::Tilde => ("~", " "),
        Selection::Brackets => ("[", "]"),
        Selection::Outline => (" ", " "),
    };

    let (start_char, end_char) = selection;

    let result = string.replacen(" ", start_char, 1);
    result.replace(" ", end_char)
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
            // Think about getting rid of spaces, instead adding them programmatically
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
pub struct Todo {
    menu: [Menu; 4],
    selected_menu: Menu,
    selection_style: Selection,
}

impl Todo {
    pub fn init(config: &Config) -> Self {
        Todo {
            menu: [Menu::Todo, Menu::Done, Menu::Settings, Menu::Help],
            selected_menu: Menu::Todo,
            selection_style: config.selection_style.clone(),
        }
    }

    fn set_selected_menu(&mut self, menu: Menu) {
        self.selected_menu = menu;
    }

    // TODO: refactor
    fn draw_menu(&self) {
        let menu = self.menu.clone();

        let menu = menu.map(|item| {
            if item == self.selected_menu {
                return get_selected_str(item.as_str(), self.selection_style.clone());
            }

            String::from(item.as_str())
        });
        // TODO: think about a way to process Selection style changing background color
        // probably one if will be enough

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        println!("{}", menu.join("   "));
    }

    pub fn run(&mut self) {
        self.draw_menu();

        // TODO: hide and bring back cursor
        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        // TODO: improve unwraps
        // TODO: think about all the clones
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    break;
                }
                // TODO: refactor
                Key::Char('h') => {
                    let index = self
                        .menu
                        .iter()
                        .position(|item| *item == self.selected_menu)
                        .unwrap();
                    let mut chosen_menu = Menu::Todo;
                    if index > 0 {
                        chosen_menu = self.menu[index - 1].clone();
                    }

                    self.set_selected_menu(chosen_menu);
                    self.draw_menu();
                }
                Key::Char('l') => {
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
            stdout.flush().unwrap();
        }
    }
}
