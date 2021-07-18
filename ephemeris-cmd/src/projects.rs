
use ephemeris::state::State;
use prettytable::{Table, row, cell};
use prettytable::format;
use crate::*;
use crate::tags::*;

fn list_projects(state: &mut Box<State>, tagfilter: &Option<String>) {
    let mut table = Table::new();
    table.set_titles(row![bF->"Code", bF->"Name", bF->"Tags"]);
   
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    for pi in state.projects.values() {
        let p = pi.borrow();
        match tagfilter {
        Some(t) => {
            let taglist = p.tags.as_ref().unwrap();
            if !taglist.contains(&t) { continue; }
        },
        None => (),
        };
        let tagstr : String = match &p.tags {
        Some(tv) => {
            tag_to_string(&tv)                   
        },
        None => String::from(""),
        };
        table.add_row(row![p.code, p.name, tagstr]);
   
        
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

pub fn cmd_project(state: &mut Box<State>, cmd: &crate::Project) {

    match &cmd.subcmd {
        ProjectSubCommand::List(c) => {
            list_projects(state, &c.tag);
        },
        ProjectSubCommand::Add(c) => {
            let tags = c.tags.as_ref().unwrap();
            println!("Add Project");
            println!("[*] Code: {}", c.code);
            println!("[*] Name: {}", c.name);
            println!("[*] Tags: {}", tag_to_string(&tags));
        
            state.project_add(&c.code, &c.name, &c.tags).unwrap();
            list_projects(state, &None);
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
        }
    }
}
