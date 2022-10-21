use crate::app::helper::{finish_print, move_cursor};

use super::todo::{Status, TodoItem};

use chrono::Utc;

pub struct Done {
    done_list: Vec<TodoItem>,
}

impl Done {
    pub fn init(todo_list: &Vec<TodoItem>) -> Self {
        let done_list = todo_list
            .clone()
            .into_iter()
            .filter(|item| {
                let today = Utc::now().date();
                let modified = item.date_modified.date();

                item.status == Status::Done && today > modified
            })
            .collect();

        Self { done_list }
    }

    pub fn render(&self) {
        move_cursor(5, 2);
        for item in &self.done_list {
            println!("{}", item.description);
        }

        finish_print();
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    #[test]
    fn check_dates() {
        let first_date_str = "2023-01-01 15:31:01.585089387 UTC";
        let second_date_str = "2022-10-20 15:32:01.585089387 UTC";

        let first_date = first_date_str.parse::<DateTime<Utc>>().unwrap().date();
        let second_date = second_date_str.parse::<DateTime<Utc>>().unwrap().date();

        let today = Utc::now().date();

        println!(
            "FIRST DATE : {}, SECOND DATE: {}, FIRST IS MORE THAN SECOND: {}",
            first_date,
            second_date,
            first_date > second_date
        );

        // assert!(today < yesterday);
    }
}
