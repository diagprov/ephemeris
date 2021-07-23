
use std::cell::{Ref};
use prettytable::{Table, row, cell};
use prettytable::format;
use ephemeris::state::State;
use ephemeris::tasks::{Task, TaskDue};
use crate::{TaskSubCommand};
use crate::tags::*;

pub fn list_tasks(state: &mut Box<State>, tagfilter: &Option<String>) {
    
    let mut table = Table::new();
    table.set_titles(row![bF->"Hash", 
        bF->"Deadline", bF->"S", bF->"Name", 
        bF->"Project", bF->"Tags"]);
   
    // https://docs.rs/prettytable-rs/0.8.0/prettytable/format/consts/index.html
    table.set_format(*format::consts::FORMAT_CLEAN);//BOX_CHARS);
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
        let status = match task.done {
            true => String::from("✓"),
            false => String::from("𐄂"),
        };
        table.add_row(row![task.hash, due_string, status, task.name, projectname, tagstr]);
   
        
    }

    table.print_tty(true);
}

fn display_task_inner(state: &Box<State>, task: Ref<Task>, _code: &String) {
    let projectname = match &task.projectcode {
            Some(pc) => {
                match &state.projects.get(pc) {
                Some(proj) => String::from(&proj.borrow().name),
                None => String::from("Unknown project code."),
                }
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
    let status = match task.done {
        true => String::from("✓"),
        false => String::from("𐄂"),
    };
    let tagstr : String = match &task.tags {
        Some(tv) => {
            tag_to_string(&tv)                   
        },
        None => String::from(""),
        };

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Name", &task.name]);
    table.add_row(row!["Project", projectname]);
    table.add_row(row!["Due", due_string]);
    table.add_row(row!["Done", status]);
    table.add_row(row!["Tags", tagstr]);
    table.add_row(row!["Hash", &task.hash]);

    table.print_tty(true);
}

fn display_task(state: &mut Box<State>, code: &String) {

    let mut found_displayed = false;
    for t in &state.tasks {
        let ti = t.borrow();
        if &ti.hash == code {
            let task = ti;
            display_task_inner(&state, task, code);
            found_displayed = true;
            break;
        }
    }

    if !found_displayed {
        println!("Task with code {} not found.", code);
    }
}

fn complete_task(state: &mut Box<State>, code: &String) -> Result<(),String> {

    let mut found_modified = false;
    for t in &state.tasks {
        let ti = t.borrow_mut();
        if &ti.hash == code {
            let mut task = ti;
            task.mark_done();
            found_modified = true;
            break;
        }
    }
    if found_modified {
        state.save()
    } else {
        Err(format!("Task with code {} not found.", code))
    }
}

pub fn cmd_tasks(state: &mut Box<State>, cmd: &crate::Task) {
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
            state.save().unwrap();
        }
        TaskSubCommand::Remove(c) => {
            match state.task_remove(&c.hash) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error occurred: {}", e);
                return;
            },
            };
            match state.save() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error occurred: {}", e);
                return;
            },
            };
        },
        TaskSubCommand::Show(c) => {
            display_task(state, &c.hash);         
        },
        TaskSubCommand::Done(c) => {
            match complete_task(state, &c.hash) {
            Ok(_) => display_task(state, &c.hash),
            Err(e) => {
                eprintln!("Error occurred: {}", e);
                return;
            },
            }
        },
        TaskSubCommand::Hash => {
            match Task::genhashcode() {
            Ok(hc) => println!("{}", hc),
            Err(e) => println!("Error: {:?}", e),
            }
        }
        //_ => (),
    };
}
