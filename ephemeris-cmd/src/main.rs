
#![macro_use]
extern crate ephemeris;
extern crate rustyline;
extern crate human_panic;
// simplest method of use, but sacrifices some flexibility.
use human_panic::setup_panic;
use clap::{AppSettings, Args, Parser, Subcommand};
use ephemeris::state::State;

mod projects;
mod repl;
mod tags;
mod tasks;
mod time;


use crate::projects::*;
use crate::tasks::*;
use crate::repl::*;
use crate::time::*;

/// Ephemeris is a Task and Simple Project Management utility 
#[derive(Parser)]
#[clap(name="Ephemeris", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting(AppSettings::ColoredHelp))]
struct EphemerisArgs {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Project(Project),
    Task(Task),
    Shell(Shell),
    Time(Time),
    /// Validate the database to ensure there are no issues.
    Validate,

}

/// Invoke an interactive shell. 
#[derive(Args)]
#[clap(name="Ephemeris Project Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Shell {
}

/// Modify or view Projects
#[derive(Args)]
#[clap(name="Ephemeris Project Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Project {
    #[clap(subcommand)]
    pub subcmd: ProjectSubCommand,
}

/// Modify or view Tasks
#[derive(Args)]
#[clap(name="Ephemeris Task Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Task {
    #[clap(subcommand)]
    pub subcmd: TaskSubCommand,
}

#[derive(Args)]
#[clap(name="Ephemeris Time Tools", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Time {
    #[clap(subcommand)]
    pub subcmd: TimeSubCommand,
}


#[derive(Subcommand)]
pub enum ProjectSubCommand {
    List(ProjectList),
    Add(ProjectAdd),
    Remove(ProjectRemove),
    Show(ProjectShow),
    Tasks(ProjectTasks),
}

#[derive(Subcommand)]
pub enum TaskSubCommand {
    List(TaskList),
    Add(TaskAdd),
    Show(TaskShow),
    Done(TaskDone),
    Remove(TaskRemove),
    /// Generate a hash shortcode for editing files by hand.
    Hash,
}

#[derive(Subcommand)]
pub enum TimeSubCommand {
    Test(TimeTest),
}


/// List Projects
#[derive(Args)]
#[clap(name = "list")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectList {
    #[clap(long)]
    tag: Option<String>,
    /// Display all projects, even those with no tasks assigned.
    #[clap(long)]
    with_no_tasks : bool,
}

/// Show a given project
#[derive(Args)]
#[clap(name = "project")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectShow {
    code: String,
}

/// Show a given project
#[derive(Args)]
#[clap(name = "project")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectTasks {
    code: String,
}



/// Add a new Project
#[derive(Args)]
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
#[derive(Args)]
#[clap(name = "remove")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct ProjectRemove {
    code: String,
}

/// List Tasks
#[derive(Args)]
#[clap(name = "list")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TaskList {
    //#[clap(long)]
    tag: Option<String>,
}

/// Add a new Task
#[derive(Args)]
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

/// Remove a given task
#[derive(Args)]
#[clap(name = "task")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TaskRemove {
    hash: String,
}

/// Show a given task
#[derive(Args)]
#[clap(name = "task")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TaskShow {
    hash: String,
}

/// Show a given task
#[derive(Args)]
#[clap(name = "task")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TaskDone {
    hash: String,
}


/// Show a given task
#[derive(Args)]
#[clap(name = "time")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct TimeTest {
}

fn main() {
    setup_panic!();

    let args = EphemerisArgs::parse();
    let mut state : Box<State> = match State::load() {
    Ok(s) => s,
    Err(e) => {
        eprintln!("An error occurred loading state: {}", e);
        eprintln!("Please run init, make sure ~/.ephemeris exists, or set EPHEMERIS_DIR");
        return;
    },
    };

    match args.subcmd {
        Commands::Project(p) => cmd_project(&mut state, &p),
        Commands::Task(p) => cmd_tasks(&mut state, &p),
        Commands::Shell(_) => repl(&mut state),
        Commands::Time(t) => cmd_time(&mut state, &t),
        Commands::Validate => println!("Validation Requested."),
    };
}
