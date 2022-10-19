// TODO: refactor imports
use crate::todo::helper::{finish_print, hide_cursor, move_cursor};
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

use linefeed::{Interface, Prompter, ReadResult};
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result as RSResult};

use super::helper::{show_blinking_cursor, show_cursor};

const DELIMITER: &'static str = "|";
const WRAPPER: &'static str = " ";
const LIST_MARK: &'static str = "•";
const LIST_SPACING: &'static str = " ";
const LIST_LEFT_MARGIN: u16 = 2;
const LIST_TOP_MARGIN: u16 = 2;

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

impl TodoItem {
    fn format_description(description: &str) -> String {
        format!("{WRAPPER}{LIST_MARK}{LIST_SPACING}{description}{WRAPPER}")
    }
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
            // TODO: id is probably not needed at all
            let id = item_config[0].parse::<u16>().expect("Couldn't parse todo.txt id to u32, check that all ids are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let date_created = item_config[1].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let date_modified = item_config[2].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let status = Status::from_str(&item_config[3]).unwrap_or_else(|err| {
                eprintln!("Couldn't get todo item status: {err}. Setting to default status 'Todo'");
                Status::Todo
            });
            let description = item_config[4].to_owned();

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
            move_cursor(LIST_LEFT_MARGIN, cursor_y);

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

            let text = TodoItem::format_description(&item.description);

            Selection::print_styled(text.as_str(), print_style);

            cursor_y += 1;
        }

        finish_print();
        // TODO: also add initialization of `todo.txt` list in config!!
    }

    pub fn listen_keys(&mut self, key: &Key) {
        match key {
            // TODO: maybe there is a way to check keys simplier
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::Up) => {
                if self.selected_index != 0 {
                    self.selected_index -= 1;
                }
            }
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::Down) => {
                if self.todo_list.len() > 0
                    && self.selected_index < (self.todo_list.len() - 1) as u16
                {
                    self.selected_index += 1;
                }
            }
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::AddTodo) => {
                let x_offset = LIST_LEFT_MARGIN
                    + DELIMITER.len() as u16
                    + WRAPPER.len() as u16
                    + LIST_SPACING.len() as u16;
                let y_offset = self.todo_list.len() as u16 + LIST_TOP_MARGIN;

                show_cursor();
                move_cursor(x_offset, y_offset);

                // TODO: remove all unwraps on the page
                let interface = Interface::new("todo_add").unwrap();

                // Probably change it
                while let ReadResult::Input(line) = interface.read_line().unwrap() {
                    let todo_item = TodoItem {
                        id: self.todo_list.len() as u16 + 1,
                        date_created: Utc::now(),
                        date_modified: Utc::now(),
                        status: Status::Todo,
                        description: line.trim().to_owned(),
                    };

                    if todo_item.description.len() > 0 {
                        self.todo_list.push(todo_item);
                    }

                    hide_cursor();
                    break;
                }
            }
            Key::Char(ch)
                if ch == &Action::get_action_char(self.key_mapping, Action::RemoveTodo) =>
            {
                if self.key_mapping.len() < 1 {
                    return;
                }

                let list: Vec<TodoItem> = self
                    .todo_list
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|(index, _)| *index as u16 != self.selected_index)
                    .map(|(_, item)| item)
                    .collect();

                if self.selected_index > 0 && self.selected_index > list.len() as u16 - 1 {
                    self.selected_index -= 1;
                }

                self.todo_list = list;
            }
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::Mark) => {
                let list: Vec<TodoItem> = self
                    .todo_list
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(index, mut item)| {
                        if index as u16 == self.selected_index {
                            item.status = match item.status {
                                Status::Done => Status::Todo,
                                Status::Todo => Status::Done,
                            }
                        }
                        item
                    })
                    .collect();

                self.todo_list = list;
            }
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::EditTodo) => {
                // TODO: start off here
                let left_thing = format!("  {LIST_MARK}{WRAPPER}{LIST_SPACING}");
                // let x_offset = LIST_LEFT_MARGIN
                //     + DELIMITER.len() as u16
                //     + WRAPPER.len() as u16
                //     + LIST_SPACING.len() as u16;
                let x_offset = left_thing.len() as u16;
                let y_offset = self.selected_index as u16 + LIST_TOP_MARGIN;

                show_blinking_cursor();
                move_cursor(x_offset, y_offset);

                // let interface = Interface::new("todo_edit").unwrap();
                let description = &self.todo_list[self.selected_index as usize].description;
                let description = format!(" {description} ");

                let mut rl = Editor::<()>::new().unwrap();

                while let RSResult::Ok(line) = rl.readline_with_initial(&left_thing, (&description, "")) {
                    self.todo_list[self.selected_index as usize].description =
                        line.trim().to_owned();
                    break;
                }
                // interface.set_buffer(&description).unwrap();

                // while let ReadResult::Input(line) = interface.read_line().unwrap() {
                //     match line {
                //         _ => self.render(),
                //     }
                //     self.todo_list[self.selected_index as usize].description =
                //         line.trim().to_owned();
                //     break;
                // }
                
                
                
                // TODO: add a check if result message is empty

                hide_cursor();
            }
            _ => {}
        }
    }
}
