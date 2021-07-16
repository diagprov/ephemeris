
extern crate clap;
extern crate ephemeris;
extern crate rustyline;
// simplest method of use, but sacrifices some flexibility.
use clap::{AppSettings, Clap};
use ephemeris::state::State;

mod projects;
mod repl;
mod tags;
mod tasks;


use crate::projects::*;
use crate::tasks::*;
use crate::repl::*;

/// Ephemeris is a Task and Simple Project Management utility 
#[derive(Clap)]
#[clap(name="Ephemeris", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct EphemerisArgs {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Project(Project),
    Task(Task),
    Shell(Shell),
}

/// Invoke an interactive shell. 
#[derive(Clap)]
#[clap(name="Ephemeris Project Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Shell {
}

/// Modify or view Projects
#[derive(Clap)]
#[clap(name="Ephemeris Project Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Project {
    #[clap(subcommand)]
    pub subcmd: ProjectSubCommand,
}

/// Modify or view Tasks
#[derive(Clap)]
#[clap(name="Ephemeris Task Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Task {
    #[clap(subcommand)]
    pub subcmd: TaskSubCommand,
}


#[derive(Clap)]
pub enum ProjectSubCommand {
    List(ProjectList),
    Add(ProjectAdd),
    Remove(ProjectRemove),
    Show(ProjectShow),
}

#[derive(Clap)]
pub enum TaskSubCommand {
    List(TaskList),
    Add(TaskAdd),
    Show(TaskShow),
    /// Generate a hash shortcode for editing files by hand.
    Hash,
    //Remove(TaskRemove),
}


/// List Projects
#[derive(Clap)]
#[clap(name = "list")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectList {
    #[clap(long)]
    tag: Option<String>,
}

/// Show a given project
#[derive(Clap)]
#[clap(name = "project")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectShow {
    code: String,
}


/// Add a new Project
#[derive(Clap)]
#[clap(name = "add")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectAdd {
    #[clap(short, long)]
    code: String,
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    tags: Option<Vec<String>>,
}

/// Remove a Project
#[derive(Clap)]
#[clap(name = "remove")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectRemove {
    #[clap(short, long)]
    code: String,
}

/// List Tasks
#[derive(Clap)]
#[clap(name = "list")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TaskList {
    #[clap(long)]
    tag: Option<String>,
}

/// Add a new Task
#[derive(Clap)]
#[clap(name = "list")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TaskAdd {
    #[clap(short, long)]
    projectcode: Option<String>,
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    tags: Option<Vec<String>>,
    #[clap(short, long)]
    due: Option<String>,
}

/// Show a given task
#[derive(Clap)]
#[clap(name = "task")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TaskShow {
    hash: String,
}

fn main() {

    let args = EphemerisArgs::parse();
    let mut state : Box<State> = State::load().unwrap();

    match args.subcmd {
        SubCommand::Project(p) => cmd_project(&mut state, &p),
        SubCommand::Task(p) => cmd_tasks(&mut state, &p),
        SubCommand::Shell(_) => repl(&mut state),
    };
}
