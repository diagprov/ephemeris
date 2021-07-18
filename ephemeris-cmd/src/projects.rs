
use ephemeris::state::{State};
use ephemeris::tasks::{TaskRef, TaskDue};
use prettytable::{Table, row, cell};
use prettytable::format;
use crate::*;
use crate::tags::*;

fn list_projects(state: &mut Box<State>, tagfilter: &Option<String>, withNoTasks: bool) {
    let mut table = Table::new();
    table.set_titles(row![bF->"Code", bF->"Name", bF->"Tags", bF->"Task Count"]);
   
    // https://docs.rs/prettytable-rs/0.8.0/prettytable/format/consts/index.html
    table.set_format(*format::consts::FORMAT_CLEAN);//BOX_CHARS);
    for pi in state.projects.values() {
        let p = pi.borrow();
        match tagfilter {
        Some(t) => {
            let taglist = p.tags.as_ref().unwrap();
            if !taglist.contains(&t) { continue; }
        },
        None => (),
        };

        let code = &p.code;
        let mut taskcount : usize = 0;
        if state.taskmap.contains_key(code) {
            taskcount = match state.taskmap.get(code) {
                Some(v) => v.len(),
                None => 0,
            };
        }
        if withNoTasks == false && taskcount == 0 {
            continue;
        }

        let tagstr : String = match &p.tags {
        Some(tv) => {
            tag_to_string(&tv)                   
        },
        None => String::from(""),
        };
        table.add_row(row![p.code, p.name, tagstr, taskcount]);
   
        
    }

    table.print_tty(true);
}

fn display_project(state: &mut Box<State>, code: &String) {


    let project = match state.projects.get(code) {
        Some(p) => p.borrow(),
        None => {
            println!("No project with code {}", code);
            return;
        },
    };
    let tagstr : String = match &project.tags {
        Some(tv) => {
            tag_to_string(&tv)                   
        },
        None => String::from(""),
    };
    

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Code", &project.code]);
    table.add_row(row!["Name", &project.name]);
    table.add_row(row!["Tags", tagstr]);
    table.print_tty(true);
}

fn list_project_tasks(state: &mut Box<State>, projectcode: &String) {
    
    let mut table = Table::new();
    table.set_titles(row![bF->"Hash", 
        bF->"Deadline", bF->"S", bF->"Name", 
        bF->"Project", bF->"Tags"]);
   
    let tasklist : &Vec<TaskRef> = match state.taskmap.get(projectcode) {
    Some(v) => v,
    None => {
        println!("Project code not found.");
        return;
    },
    };

    // https://docs.rs/prettytable-rs/0.8.0/prettytable/format/consts/index.html
    table.set_format(*format::consts::FORMAT_CLEAN);//BOX_CHARS);
    for taski in tasklist {
        let task = taski.borrow();
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
        let status = match task.done {
            true => String::from("✓"),
            false => String::from("𐄂"),
        };
        table.add_row(row![task.hash, due_string, status, task.name, tagstr]);
   
        
    }

    table.print_tty(true);
}



pub fn cmd_project(state: &mut Box<State>, cmd: &crate::Project) {

    match &cmd.subcmd {
        ProjectSubCommand::List(c) => {
            list_projects(state, &c.tag, c.withNoTasks);
        },
        ProjectSubCommand::Add(c) => {
            let tags = c.tags.as_ref().unwrap();
            println!("Add Project");
            println!("[*] Code: {}", c.code);
            println!("[*] Name: {}", c.name);
            println!("[*] Tags: {}", tag_to_string(&tags));
        
            state.project_add(&c.code, &c.name, &c.tags).unwrap();
            //list_projects(state, &None);
            state.save().unwrap();
        },
        ProjectSubCommand::Remove(c) => {
            match state.project_remove(&c.code) {
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
        ProjectSubCommand::Show(c) => {
            display_project(state, &c.code)
        },
        ProjectSubCommand::Tasks(c) => {
            list_project_tasks(state, &c.code)
        }
    }
}
