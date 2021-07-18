
use std::cell::RefCell;
use std::rc::Rc;
use bs58;
use getrandom;
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
    pub done: bool,
    pub projectcode : Option<String>,
    pub tags : Option<Vec<String>>,
    pub hash: String,
    pub due: Option<TaskDue>,
}


#[derive(Clone, Serialize, Deserialize)]
pub struct TaskList {
    pub task : Option<Vec<TaskRef>>,
}


impl Task {

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
