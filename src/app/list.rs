// TODO: refactor imports
use crate::app::helper::{finish_print, hide_cursor, move_cursor};
use crate::app::selection::PrintStyle;
use crate::app::selection::Selection;
use crate::app::Action;

use std::fs::{write, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

use core::str::FromStr;

use chrono::{DateTime, Utc};

use termion;
use termion::event::Key;

use rustyline::{Editor, Result as RLResult};

use super::helper::{show_blinking_cursor, show_cursor};

const DELIMITER: &'static str = "|";
const WRAPPER: &'static str = " ";
const LIST_MARK: &'static str = "·";
const LIST_MARK_SELECTED: &'static str = "•";
const LIST_SPACING: &'static str = " ";
const LIST_LEFT_MARGIN: &'static str = "  ";
const LIST_TOP_MARGIN: u16 = 2;

// TODO: add emojis in the future
// TODO: think about page scroll when many todos
// TODO: think about line return if text doesn't fit or set maximum text length

fn read_list<'a>(
    path: &'a Path,
    selection_style: &'a Selection,
    key_mapping: &'a Vec<(Action, char)>,
) -> List<'a> {
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
            panic!("todo.txt file seems to be corrupted: App configuration doesn't match. Check that configuration matches 'id|DateTime|DateTime|Status|Description' pattern or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
        }

        // TODO: do sth with repetetive messages
        // TODO: id is probably not needed at all
        // TODO: also date_modified is currently not used
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
        path,
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
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

#[derive(Debug, Clone)]
pub struct List<'a> {
    todo_list: Vec<TodoItem>,
    key_mapping: &'a Vec<(Action, char)>,
    selection_style: &'a Selection,
    selected_index: u16,
    path: &'a Path,
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

        read_list(&path, &selection_style, &key_mapping)
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

        write(self.path, contents.join("\n")).unwrap_or_else(|err| {
            panic!("Couldn't save updated todo list: {err}");
        });
    }

    pub fn render(&self) {
        let mut cursor_y = 2;

        for item in &self.todo_list {
            move_cursor(LIST_LEFT_MARGIN.len() as u16, cursor_y);

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
                let prompt = format!("{LIST_LEFT_MARGIN}{LIST_MARK_SELECTED}{WRAPPER}");
                let x_offset = prompt.len() as u16;
                let y_offset = self.todo_list.len() as u16 + LIST_TOP_MARGIN;

                show_cursor();
                move_cursor(x_offset, y_offset);

                let mut rl = Editor::<()>::new().unwrap();

                while let RLResult::Ok(line) = rl.readline(&prompt) {
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

                    break;
                }

                hide_cursor();
                self.write();
            }
            Key::Char(ch)
                if ch == &Action::get_action_char(self.key_mapping, Action::RemoveTodo) =>
            {
                self.remove_selected_todo();
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

                let prompt = format!("{LIST_LEFT_MARGIN}{LIST_MARK_SELECTED}{WRAPPER}");
                let x_offset = prompt.len() as u16;
                let y_offset = self.selected_index as u16 + LIST_TOP_MARGIN;

                show_blinking_cursor();
                move_cursor(x_offset, y_offset);

                let description = &self.todo_list[self.selected_index as usize].description;

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

                hide_cursor();
                self.write();
            }
            _ => {}
        }
    }
}
