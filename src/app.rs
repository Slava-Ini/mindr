pub mod helper;
pub mod list;
pub mod menu;
pub mod selection;

use std::io::{stdin, stdout, Write};
use std::path::Path;

use crate::app::helper::{hide_cursor, prepare_print, show_cursor};
use crate::app::list::List;
use crate::app::menu::Menu;
use crate::config::Action;
use crate::config::Config;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct App<'a> {
    menu: Menu<'a>,
    list: List<'a>,
    key_mapping: &'a Vec<(Action, char)>,
}

impl<'a> App<'a> {
    // TODO: make path better
    pub fn init(config: &'a Config, path: &'a Path) -> Self {
        // TODO: maybe there is a way to get rid of &config.key_mapping somewhow maybe just pass a
        // whole config or not
        let menu = Menu::init(&config.selection_style, &config.key_mapping);
        let list = List::init(&config.selection_style, &config.key_mapping, &path);

        App {
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
