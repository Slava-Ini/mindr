// -- Terminal Size --
// println!("{:?}", termion::terminal_size().unwrap());
// -- Cursor --
// termion::cursor::Goto(5, 10);
// -- Clear --
// print!("{}", termion::clear::All);
// println!("{}", termion::cursor::Show);
pub mod helper;
pub mod list;
pub mod menu;
pub mod selection;

use std::io::{stdin, stdout};

use crate::config::Action;
use crate::config::Config;
use crate::todo::helper::finish_print;
use crate::todo::helper::prepare_print;
use crate::todo::menu::Menu;
use crate::todo::list::List;

use termion;
use termion::raw::IntoRawMode;

#[derive(Clone)]
pub struct Todo<'a> {
    menu: Menu<'a>,
    list: List,
}

impl<'a> Todo<'a> {
    pub fn init(config: &'a Config) -> Self {
        let menu = Menu::init(&config.selection_style, &config.key_mapping);
        let list = List::init();

        Todo {
            menu,
            list,
        }
    }

    pub fn run(&mut self) {
        print!("{}", termion::cursor::Hide);
        prepare_print();

        let stdin = stdin();
        let screen = termion::screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());

        self.menu.render();
        self.list.render();
        self.menu.listen_keys(stdin, screen);

        finish_print();
        print!("{}", termion::cursor::Show);
    }
}
