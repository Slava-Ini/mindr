// TODO: refactor imports
use crate::todo::helper::{finish_print, move_cursor};
use crate::todo::selection::PrintStyle;
use crate::todo::selection::Selection;
use crate::todo::Action;

use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use core::str::FromStr;

use chrono::{DateTime, Utc};

use termion;
use termion::event::Key;

const DELIMITER: &'static str = "|";
const WRAPPER: &'static str = " ";
const LIST_MARK: &'static str = "•";
const LIST_SPACING: &'static str = " ";
const CURSOR_MARGIN: u16 = 2;

// TODO: add emojis in the future
// TODO: think about page scroll when many todos
// TODO: think about line return if text doesn't fit or set maximum text length

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Todo,
    Done,
}

impl FromStr for Status {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(Status::Todo),
            "Done" => Ok(Status::Done),
            _ => return Err("No such status availabe, try using 'Todo/Done'"),
        }
    }
}

// TODO: remove all debug derivatives
#[derive(Debug, Clone, PartialEq)]
struct TodoItem {
    id: u16,
    date_created: DateTime<Utc>,
    date_modified: DateTime<Utc>,
    status: Status,
    description: String,
}

// TODO: work on paths and where to store them
fn get_list_path() -> PathBuf {
    let user_name = env::var("USERNAME").expect("Couldn't get system user");

    let path: PathBuf = ["/home", user_name.as_str(), ".config", "mindr", "todo.txt"]
        .iter()
        .collect();

    path
}

#[derive(Debug, Clone)]
pub struct List<'a> {
    todo_list: Vec<TodoItem>,
    key_mapping: &'a Vec<(Action, char)>,
    selection_style: &'a Selection,
    selected_index: u16,
}

// TODO: think where to put this fn
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl<'a> List<'a> {
    pub fn init(selection_style: &'a Selection, key_mapping: &'a Vec<(Action, char)>) -> Self {
        let path = get_list_path();
        // let dt = Utc::now().to_string();
        // let dt_from_str = dt.parse::<DateTime<Utc>>().unwrap();

        let lines = match read_lines(&path) {
            Ok(lines) => lines,
            Err(error) => {
                panic!("Couldn't read todo.txt file: {error}");
            }
        };

        let mut todo_list: Vec<TodoItem> = Vec::new();

        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    panic!("todo.txt file seems to be corrupted: {error}. Try deleting the file and restarting mindr. NOTE: deleting the file will destroy user's todo list data");
                }
            };

            let item_config = line.split(DELIMITER).collect::<Vec<&str>>();

            if item_config.len() != 5 {
                panic!("todo.txt file seems to be corrupted: Todo configuration doesn't match. Check that configuration matches 'id|DateTime|DateTime|Status|Description' pattern or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            }

            // TODO: do sth with repetetive messages
            let id = item_config[0].parse::<u16>().expect("Couldn't parse todo.txt id to u32, check that all ids are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let date_created = item_config[1].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let date_modified = item_config[2].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let status = Status::from_str(&item_config[3]).unwrap_or_else(|err| {
                eprintln!("Couldn't get todo item status: {err}. Setting to default status 'Todo'");
                Status::Todo
            });
            let description = item_config[4].to_owned();
            let description = format!("{WRAPPER}{LIST_MARK}{LIST_SPACING}{description}{WRAPPER}");

            let todo_item = TodoItem {
                id,
                date_created,
                date_modified,
                status,
                description,
            };

            todo_list.push(todo_item);
        }

        List {
            todo_list,
            key_mapping,
            selection_style,
            selected_index: 0,
        }
    }

    fn write(&self) {}

    pub fn render(&self) {
        let mut cursor_y = 2;

        for item in &self.todo_list {
            move_cursor(CURSOR_MARGIN, cursor_y);

            let index = &self.todo_list.iter().position(|todo| todo == item).unwrap();
            let selected_index = &(self.selected_index as usize);
            let selection = if selected_index == index {
                Some(self.selection_style)
            } else {
                None
            };

            let print_style = PrintStyle {
                selection,
                strikethrough: item.status == Status::Done,
                spacing: Some(LIST_SPACING),
            };

            Selection::print_styled(&item.description, print_style);

            cursor_y += 1;
        }

        finish_print();
        // TODO: also add initialization of `todo.txt` list in config!!
    }

    pub fn listen_keys(&mut self, key: &Key) {
        match key {
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::Up) => {
                if self.selected_index != 0 {
                    self.selected_index -= 1;
                }
            }
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::Down) => {
                if self.selected_index < (self.todo_list.len() - 1) as u16 {
                    self.selected_index += 1;
                }
            }
            _ => {}
        }
    }
}
