pub mod helper;
pub mod menu;
pub mod selection;
pub mod tabs;

use std::io::{stdin, stdout, Write};
use std::path::Path;

use crate::app::helper::{Cursor, Print};
use crate::app::menu::{Menu, MenuItem};
use crate::app::tabs::done::Done;
use crate::app::tabs::todo::Todo;
use crate::config::Action;
use crate::config::Config;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct App<'a> {
    menu: Menu<'a>,
    todo: Todo<'a>,
    done: Done,
    key_mapping: &'a Vec<(Action, char)>,
}

impl<'a> App<'a> {
    // TODO: make path better
    pub fn init(config: &'a Config, path: &'a Path) -> Self {
        // TODO: maybe there is a way to get rid of &config.key_mapping somewhow maybe just pass a
        // whole config or not
        let menu = Menu::init(&config.selection_style, &config.key_mapping);
        let todo = Todo::init(&config.selection_style, &config.key_mapping, &path);
        let done = Done::init(&todo.todo_list);

        App {
            menu,
            todo,
            done,
            key_mapping: &config.key_mapping,
        }
    }

    pub fn run(&mut self) {
        Cursor::hide();
        Print::prepare();

        let stdin = stdin();
        let mut screen = termion::screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());

        let keys = stdin.keys();

        // TODO: probably make into a one `self` method
        self.menu.render();
        self.todo.render();

        for key in keys {
            let key = key.unwrap();

            if key == Key::Char(Action::get_action_char(self.key_mapping, Action::Quit)) {
                break;
            }

            // TODO: rename `listen_keys`
            self.menu.listen_keys(&key);

            match self.menu.selected_menu {
                MenuItem::Todo => {
                    self.todo.listen_keys(&key);
                    self.todo.render();
                }
                MenuItem::Done => {
                    self.done.render();
                }
                _ => {}
            }

            self.menu.render();

            screen.flush().unwrap();
        }

        Cursor::show();
    }
}
