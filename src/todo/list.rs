use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use core::str::FromStr;

use chrono::{DateTime, Utc};

use super::helper::{prepare_print, finish_print};

const DELIMITER: &'static str = "|";

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
struct TodoItem {
    id: u32,
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
pub struct List {
    todo_list: Vec<TodoItem>,
}

// TODO: think where to put this fn
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl List {
    pub fn init() -> Self {
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
                    panic!("todo.txt file seems to be corrupted: {error}. Try deleting the file and restarting mindr. NOTE: NOTE: deleting the file will destroy user's todo list data");
                }
            };

            let item_config = line.split(DELIMITER).collect::<Vec<&str>>();

            if item_config.len() != 5 {
                panic!("todo.txt file seems to be corrupted: Todo configuration doesn't match. Check that configuration matches 'id|DateTime|DateTime|Status|Description' pattern or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            }

            // TODO: do sth with repetetive messages
            let id =item_config[0].parse::<u32>().expect("Couldn't parse todo.txt id to u32, check that all ids are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let date_created= item_config[1].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
            let date_modified= item_config[2].parse::<DateTime<Utc>>().expect("Coldn't parse the date in todo.txt, check that all dates are valid or delete the file and restart mindr. NOTE: deleting the file will destroy user's todo list data");
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

        List { todo_list }
    }

    fn write(&self) {}

    pub fn render(&self) {
        // prepare_print();
        for item in &self.todo_list {
            println!("{:?}", item);
        }
        // finish_print();
        // TODO: also add initialization of `todo.txt` list in config!!

       
    }
}
