
#![macro_use]
extern crate ephemeris;
extern crate rustyline;
extern crate human_panic;


use std::io;
// simplest method of use, but sacrifices some flexibility.
use human_panic::setup_panic;
use clap::{AppSettings, Args, IntoApp, Parser, Subcommand};
use clap_generate::{generate, Generator, Shell as ClapShell};
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
    Completion(Completion),
    /// Validate the database to ensure there are no issues.
    Validate,

}

/// Invoke an interactive shell. 
#[derive(Args)]
#[clap(name="Ephemeris Project Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
pub struct Shell {
}

/// Modify or view Projects
#[derive(Args)]
#[clap(name="Ephemeris Project Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
pub struct Project {
    #[clap(subcommand)]
    pub subcmd: ProjectSubCommand,
}

/// Modify or view Tasks
#[derive(Args)]
#[clap(name="Ephemeris Task Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
pub struct Task {
    #[clap(subcommand)]
    pub subcmd: TaskSubCommand,
}

#[derive(Args)]
#[clap(name="Ephemeris Time Tools", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
pub struct Time {
    #[clap(subcommand)]
    pub subcmd: TimeSubCommand,
}

#[derive(Args)]
#[clap(name="Ephemeris Completion Scripts", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
pub struct Completion {
    #[clap(subcommand)]
    pub subcmd: CompletionSubCommand,
}

#[derive(Subcommand)]
pub enum CompletionSubCommand {
    Bash,
    Zsh,
    Psh,
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
pub struct ProjectShow {
    code: String,
}

/// Show a given project
#[derive(Args)]
#[clap(name = "project")]
pub struct ProjectTasks {
    code: String,
}



/// Add a new Project
#[derive(Args)]
#[clap(name = "add")]
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
pub struct ProjectRemove {
    code: String,
}

/// List Tasks
#[derive(Args)]
#[clap(name = "list")]
pub struct TaskList {
    //#[clap(long)]
    tag: Option<String>,
}

/// Add a new Task
#[derive(Args)]
#[clap(name = "list")]
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
pub struct TaskRemove {
    hash: String,
}

/// Show a given task
#[derive(Args)]
#[clap(name = "task")]
pub struct TaskShow {
    hash: String,
}

/// Show a given task
#[derive(Args)]
#[clap(name = "task")]
pub struct TaskDone {
    hash: String,
}


/// Show a given task
#[derive(Args)]
#[clap(name = "time")]
pub struct TimeTest {
}

fn main() {
    setup_panic!();

    let args = EphemerisArgs::parse();
    let mut app = EphemerisArgs::into_app();
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
        Commands::Completion(s) => {
            match s.subcmd {
            CompletionSubCommand::Bash =>
                generate(ClapShell::Bash, &mut app, "Ephemeris", &mut io::stdout()),
            CompletionSubCommand::Zsh =>
                generate(ClapShell::Zsh, &mut app, "Ephemeris", &mut io::stdout()),
            CompletionSubCommand::Psh =>
                generate(ClapShell::PowerShell, &mut app, "Ephemeris", &mut io::stdout())
            }
        },
        Commands::Validate => println!("Validation Requested."),
    };
}
