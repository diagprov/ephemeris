
use crate::{Task, TaskSubCommand};
use crate::tags::*;
use prettytable::{Table, row, cell};
use prettytable::format;
use ephemeris::state::State;
use ephemeris::tasks::TaskDue;

fn list_tasks(state: &mut Box<State>, tagfilter: &Option<String>) {
    
    let mut table = Table::new();
    table.set_titles(row!["Deadline", "Name", "Project", "Tags"]);
    
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    for taski in &state.tasks {
        let task = taski.borrow();
        match tagfilter {
        Some(t) => {
            let taglist = task.tags.as_ref().unwrap();
            if !taglist.contains(&t) { continue; }
        },
        None => (),
        };
        let tagstr : String = match &task.tags {
        Some(tv) => {
            tag_to_string(&tv)                   
        },
        None => String::from(""),
        };

        let due_string = match &task.due {
        Some(d) => {
            match d {
            TaskDue::Day(date) => {
                date.format("%Y-%m-%d").to_string() 
            },
            TaskDue::Time(_dt) => {
                String::from("Unhandled")
            },
            }
        },
        None => String::from("Not set"),
        };
        let projectname = match &task.projectcode {
            Some(pc) => {
                match &state.projects.get(pc) {
                Some(proj) => String::from(&proj.borrow().name),
                None => String::from("Unknown project code."),
                }
            },
            None => String::from(""),
        }; 
        table.add_row(row![due_string, task.name, projectname, tagstr]);
   
        
    }

    table.print_tty(true);
}
/*
fn display_task(state: &mut Box<State>, code: &String) {

}
*/
pub fn cmd_tasks(state: &mut Box<State>, cmd: &Task) {
    match &cmd.subcmd {
        TaskSubCommand::List(c) => {
            list_tasks(state, &c.tag);
        },
        TaskSubCommand::Add(c) => {
            let tags = c.tags.as_ref().unwrap();
            let projectcode :String = match c.projectcode.as_ref() {
            Some(code) => code.clone(),
            None => String::from(""),
            };
            println!("Add Project");
            println!("[*] Project Code: {}", projectcode);
            println!("[*] Name: {}", c.name);
            println!("[*] Tags: {}", tag_to_string(&tags));
        
            state.task_add(&c.name, &c.due, &projectcode, &c.tags).unwrap();
            //list_projects(state, &None);
            //state.save().unwrap();
        },
        //_ => (),
    };
}
