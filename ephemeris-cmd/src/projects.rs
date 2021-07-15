
use ephemeris;
use prettytable::{Table, Row, Cell, row, cell};
use crate::*;


#[inline(always)]
fn tag_to_string(tlist: &Vec<String>) -> String {
    let mut s : String = String::from("");
    for t in tlist {
        if &s == "" {
            s = format!("{}", t);
        } else {
            s = format!("{}, {}", s, t);
        }
    }
    s
}

fn list_projects(state: &mut Box<ephemeris::State>, tagfilter: &Option<String>) {
    let mut table = Table::new();
    table.set_titles(row!["Code", "Name", "Tags"]);
    for p in state.projects.values() {
    
        match(tagfilter) {
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

pub fn cmd_project(state: &mut Box<ephemeris::State>, cmd: &crate::Project) {

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
        
            state.project_add(&c.code, &c.name, &c.tags);
            list_projects(state, &None);
            state.save();
        },
        ProjectSubCommand::Remove(c) => {
            state.project_remove(&c.code);
            list_projects(state, &None);
            state.save();
        },
    }
}
