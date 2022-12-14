// TODO: refactor imports
use crate::app::helper::{Cursor, Print, Screen};
use crate::app::selection::PrintStyle;
use crate::app::selection::Selection;
use crate::app::Action;

use std::fs::{write, File};
use std::io::{self, BufRead};
use std::path::Path;

use core::str::FromStr;

use chrono::{DateTime, Utc};

use termion;
use termion::event::Key;

use rustyline::{Editor, Result as RLResult};

const DELIMITER: &'static str = "|";
const WRAPPER: &'static str = " ";
const LIST_MARK: &'static str = "·";
const LIST_MARK_SELECTED: &'static str = "•";
const LIST_SPACING: &'static str = " ";
const LIST_LEFT_MARGIN: &'static str = "  ";
const LIST_TOP_MARGIN: u16 = 2;

// TODO: add emojis in the future
// TODO: think about page scroll when many todos

fn generate_id(todo_list: &Vec<TodoItem>) -> u16 {
    let mut ids: Vec<u16> = Vec::new();

    for item in todo_list {
        ids.push(item.id);
    }

    ids.sort();

    for i in u16::MIN..u16::MAX {
        if !ids.contains(&i) {
            return i;
        }
    }

    panic!("Too many todo items");
}

fn read_todo<'a>(
    path: &'a Path,
    selection_style: &'a Selection,
    key_mapping: &'a Vec<(Action, char)>,
) -> Todo<'a> {
    let file = File::open(path).unwrap_or_else(|error| {
        panic!("Couldn't read todo.txt file: {error}");
    });
    let lines = io::BufReader::new(file).lines();

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
            panic!("todo.txt file seems to be corrupted: App configuration doesn't match. Check that configuration matches 'id|DateTime|DateTime|Status|Description' pattern or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
        }

        // TODO: do sth with repetetive messages
        // TODO: id is probably not needed at all
        // TODO: also date_modified is currently not used
        let date_modified = item_config[2].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");

        // TODO: first fix write
        // let today = Utc::now().date();
        // let modified = date_modified.date();

        // if today > modified {
        //     continue;
        // }

        let id = item_config[0].parse::<u16>().expect("Couldn't parse todo.txt id to u32, check that all ids are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
        let date_created = item_config[1].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
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

    Todo {
        todo_list,
        key_mapping,
        selection_style,
        selected_index: 0,
        path,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Todo,
    Done,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Todo => "Todo",
            Status::Done => "Done",
        }
    }
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
// TODO: item (as well as Status) is used in both `todo.rs` an `done.rs` maybe it should be in a separate mod
#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    id: u16,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub status: Status,
    pub description: String,
}

impl TodoItem {
    fn format_description(description: &str) -> String {
        format!("{WRAPPER}{LIST_MARK}{LIST_SPACING}{description}{WRAPPER}")
    }

    fn get_line_height(text: &str) -> u16 {
        let (x_size, _) = Screen::get_size();
        let y_offset = (text.len() as f32) / (x_size as f32);

        y_offset.ceil() as u16
    }
}

#[derive(Debug, Clone)]
pub struct Todo<'a> {
    pub todo_list: Vec<TodoItem>,
    key_mapping: &'a Vec<(Action, char)>,
    selection_style: &'a Selection,
    selected_index: u16,
    path: &'a Path,
}

impl<'a> Todo<'a> {
    // TODO: somehow improve amount of args and path
    pub fn init(
        selection_style: &'a Selection,
        key_mapping: &'a Vec<(Action, char)>,
        path: &'a Path,
    ) -> Self {
        if !path.exists() {
            File::create(path).expect("Couldn't create todo list storage file");

            return Self {
                key_mapping,
                selection_style,
                todo_list: Vec::new(),
                selected_index: 0,
                path,
            };
        }

        read_todo(&path, &selection_style, &key_mapping)
    }

    fn remove_selected_todo(&mut self) {
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

    // TODO: probably rename to save
    fn write(&self) {
        // TODO: implement writing on lines
        let contents = &self.todo_list;
        let contents: Vec<String> = contents
            .into_iter()
            .map(
                |TodoItem {
                     id,
                     date_created,
                     date_modified,
                     status,
                     description,
                 }| {
                    format!(
                        "{id}|{date_created}|{date_modified}|{status}|{description}",
                        status = status.as_str()
                    )
                },
            )
            .collect();

        // TODO: consider using File::write()
        write(self.path, contents.join("\n")).unwrap_or_else(|err| {
            panic!("Couldn't save updated todo list: {err}");
        });
    }

    pub fn render(&self) {
        let mut cursor_y = 2;

        for item in &self.todo_list {
            Cursor::place(LIST_LEFT_MARGIN.len() as u16, cursor_y);

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
            let y_offset = TodoItem::get_line_height(&text);

            cursor_y += y_offset;

            Selection::print_styled(text.as_str(), print_style);
        }

        Print::finsih();
    }

    pub fn listen_keys(&mut self, key: &Key) {
        match key {
            // TODO: Maybe make a macro (learn more about macros)
            // https://stackoverflow.com/questions/63876773/how-complicated-can-a-match-pattern-be-trying-to-convert-macro-from-termion-to
            Key::Char(ch) if ch == &Action::Up.as_char(self.key_mapping) => {
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
                let prompt = format!("{LIST_LEFT_MARGIN}{LIST_MARK_SELECTED}{WRAPPER}");
                let x_offset = prompt.len() as u16;
                // --- Note ---
                // We get offset to the last todo item and put cursor under it (+ 1)
                let y_offset = self.get_y_offset(self.todo_list.last().unwrap().id) + 1;

                Cursor::show();
                Cursor::place(x_offset, y_offset);

                let mut rl = Editor::<()>::new().unwrap();

                while let RLResult::Ok(line) = rl.readline(&prompt) {
                    if line.len() > 0 {
                        let todo_item = TodoItem {
                            id: generate_id(&self.todo_list),
                            date_created: Utc::now(),
                            date_modified: Utc::now(),
                            status: Status::Todo,
                            description: line.trim().to_owned(),
                        };

                        self.todo_list.push(todo_item);
                        self.write();
                    }

                    break;
                }

                Cursor::hide();
                Screen::clear();
            }
            Key::Char(ch)
                if ch == &Action::get_action_char(self.key_mapping, Action::RemoveTodo) =>
            {
                self.remove_selected_todo();
                self.write();

                Screen::clear();
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
                self.write();
            }
            Key::Char(ch) if ch == &Action::get_action_char(self.key_mapping, Action::EditTodo) => {
                // TODO: add update of date modified
                if self.todo_list.len() == 0 {
                    return;
                }

                let selected_todo = &self.todo_list[self.selected_index as usize];

                let prompt = format!("{LIST_LEFT_MARGIN}{LIST_MARK_SELECTED}{WRAPPER}");
                let x_offset = prompt.len() as u16;
                let y_offset = self.get_y_offset(selected_todo.id);

                Cursor::show();
                Cursor::place(x_offset, y_offset);

                let description = &selected_todo.description;

                let mut rl = Editor::<()>::new().unwrap();

                while let RLResult::Ok(line) = rl.readline_with_initial(&prompt, (&description, ""))
                {
                    if line.trim().len() > 0 {
                        self.todo_list[self.selected_index as usize].description =
                            line.trim().to_owned();
                    } else {
                        self.remove_selected_todo();
                    }
                    break;
                }

                Cursor::hide();
                self.write();
            }
            _ => {}
        }
    }

    fn get_y_offset(&self, to_element_id: u16) -> u16 {
        let mut offset: u16 = 0;

        self.todo_list.iter().any(|todo_item| {
            let text = TodoItem::format_description(&todo_item.description);
            let is_match = todo_item.id == to_element_id;

            if !is_match {
                offset += TodoItem::get_line_height(&text);
            }

            is_match
        });

        offset + LIST_TOP_MARGIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_intermediate_id() {
        let todo_list = vec![
            TodoItem {
                id: 0,
                date_created: Utc::now(),
                date_modified: Utc::now(),
                status: Status::Todo,
                description: String::from("Zero"),
            },
            TodoItem {
                id: 2,
                date_created: Utc::now(),
                date_modified: Utc::now(),
                status: Status::Todo,
                description: String::from("Two"),
            },
        ];

        assert_eq!(generate_id(&todo_list), 1);
    }

    #[test]
    fn test_generate_starting_id() {
        let todo_list = vec![
            TodoItem {
                id: 1,
                date_created: Utc::now(),
                date_modified: Utc::now(),
                status: Status::Todo,
                description: String::from("One"),
            },
            TodoItem {
                id: 2,
                date_created: Utc::now(),
                date_modified: Utc::now(),
                status: Status::Todo,
                description: String::from("Two"),
            },
        ];

        assert_eq!(generate_id(&todo_list), 0);
    }

    #[test]
    fn test_generate_ending_id() {
        let todo_list = vec![
            TodoItem {
                id: 0,
                date_created: Utc::now(),
                date_modified: Utc::now(),
                status: Status::Todo,
                description: String::from("Zero"),
            },
            TodoItem {
                id: 1,
                date_created: Utc::now(),
                date_modified: Utc::now(),
                status: Status::Todo,
                description: String::from("One"),
            },
        ];

        assert_eq!(generate_id(&todo_list), 2);
    }
}
