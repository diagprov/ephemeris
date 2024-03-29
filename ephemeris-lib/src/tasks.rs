
use std::cell::RefCell;
use std::rc::Rc;
use bs58;
use getrandom;
use chrono::{NaiveDate, DateTime, Utc};
#[allow(unused_imports)]
use tz::{DateTime as TzDateTime, TimeZone, UtcDateTime};
#[allow(unused_imports)]
use tzdb::{time_zone, tz_by_name};
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
    pub done: bool,
    pub projectcode : Option<String>,
    pub tags : Option<Vec<String>>,
    pub hash: String,
    pub due: Option<TaskDue>,
    pub priority: Option<u16>,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub task : Option<Vec<TaskRef>>,
}

impl Task {

    pub fn new(name: String, 
               projectcode: Option<String>, 
               tags: Option<Vec<String>>,
               _due: Option<String>,
               priority: Option<u16>,
               )  -> Result<Task, String> {
        Ok(Task {
            name: name.clone(),
            done: false,
            projectcode: projectcode.clone(),
            tags: tags.clone(),
            hash: Task::genhashcode()?,
            due: None,
            priority: priority,
        })
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
    pub fn mark_todo(&mut self) {
        self.done = false;
    }

    pub fn genhashcode() -> Result<String, String> {
        let mut randoms: [u8; 8] = [0;8];
        match getrandom::getrandom(&mut randoms) {
            Ok(_) => {
                Ok(bs58::encode(randoms).into_string())
            },
            Err(e) => {
                return Err(format!("OS Error code {}", e.code()));
            },
        }
    }

    pub fn generate_hash(&mut self) -> Result<(), String> {
        self.hash = Task::genhashcode().unwrap(); 
        Ok(())
    }

    pub fn set_due(&mut self) {
        self.due = Some(TaskDue::Day(NaiveDate::from_ymd(2022,02,03)));
    }
    /*pub fn due_to_localrepr(&self) -> String {
        let local : DateTime<Local> = DateTime::<Local>::from(self.due);
    }*/
}
