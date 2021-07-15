
extern crate ephemeris;
// simplest method of use, but sacrifices some flexibility.
use clap::{AppSettings, Clap};

mod projects;
use crate::projects::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
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
}

#[derive(Clap)]
#[clap(name="Ephemeris Project Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Project {
    #[clap(subcommand)]
    subcmd: ProjectSubCommand,
}
#[derive(Clap)]
#[clap(name="Ephemeris Task Management", version = "1.0", author = "Antony Vennard <antony@vennard.ch>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Task {
    #[clap(subcommand)]
    subcmd: TaskSubCommand,
}


#[derive(Clap)]
pub enum ProjectSubCommand {
    List(ProjectList),
    Add(ProjectAdd),
    Remove(ProjectRemove),
}

#[derive(Clap)]
pub enum TaskSubCommand {
    List(ProjectList),
}


#[derive(Clap)]
#[clap(name = "list")]
pub struct ProjectList {
    #[clap(long)]
    tag: Option<String>,
}

#[derive(Clap)]
#[clap(name = "add")]
pub struct ProjectAdd {
    #[clap(short, long)]
    code: String,
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    tags: Option<Vec<String>>,
}

#[derive(Clap)]
#[clap(name = "list")]
pub struct ProjectRemove {
    #[clap(short, long)]
    code: String,
}
fn main() {

    let args = EphemerisArgs::parse();
    let mut state : Box<ephemeris::State> = ephemeris::State::load().unwrap();

    match args.subcmd {
        SubCommand::Project(p) => cmd_project(&mut state, &p),
        SubCommand::Task(p) => println!("Task"),
    };
}
