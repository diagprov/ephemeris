
use std::cell::RefCell;
use std::rc::Rc;
use chrono::{NaiveDate, DateTime, Utc};
use serde_derive::{Serialize, Deserialize};

pub type TaskRef = Rc<RefCell<Task>>;



#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "when")]
pub enum TaskDue {
    Day(NaiveDate),
    Time(DateTime<Utc>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub name : String,
    pub projectcode : Option<String>,
    pub tags : Option<Vec<String>>,
    pub due: Option<TaskDue>,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub task : Option<Vec<TaskRef>>,
}


impl Task {

    pub fn set_due(&mut self) {
        self.due = Some(TaskDue::Day(NaiveDate::from_ymd(2022,02,03)));
    }
    /*pub fn due_to_localrepr(&self) -> String {
        let local : DateTime<Local> = DateTime::<Local>::from(self.due);
    }*/
}
