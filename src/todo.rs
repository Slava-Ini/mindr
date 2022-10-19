pub mod helper;
pub mod list;
pub mod menu;
pub mod selection;

use std::io::{stdin, stdout, Write};

use crate::config::Action;
use crate::config::Config;
use crate::todo::helper::{hide_cursor, prepare_print, show_cursor};
use crate::todo::list::List;
use crate::todo::menu::Menu;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Todo<'a> {
    menu: Menu<'a>,
    list: List<'a>,
    key_mapping: &'a Vec<(Action, char)>,
}

impl<'a> Todo<'a> {
    pub fn init(config: &'a Config) -> Self {
        // TODO: maybe there is a way to get rid of &config.key_mapping somewhow maybe just pass a
        // whole config or not
        let menu = Menu::init(&config.selection_style, &config.key_mapping);
        let list = List::init(&config.selection_style, &config.key_mapping);

        Todo {
            menu,
            list,
            key_mapping: &config.key_mapping,
        }
    }

    pub fn run(&mut self) {
        hide_cursor();
        prepare_print();

        let stdin = stdin();
        let mut screen = termion::screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());

        let keys = stdin.keys();

        // TODO: probably make into a one `self` method
        self.menu.render();
        self.list.render();

        for key in keys {
            let key = key.unwrap();

            if key == Key::Char(Action::get_action_char(self.key_mapping, Action::Quit)) {
                break;
            }

            // TODO: rename `listen_keys`
            self.menu.listen_keys(&key);
            self.list.listen_keys(&key);

            self.menu.render();
            self.list.render();

            screen.flush().unwrap();
        }

        show_cursor();
    }
}
