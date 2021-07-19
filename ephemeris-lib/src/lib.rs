
extern crate bs58;
extern crate chrono;
extern crate dirs;
extern crate getrandom;
extern crate regex;
extern crate serde_derive;
extern crate toml;

pub mod projects;
pub mod tasks;
pub mod state;
pub mod store;

pub const EPHEMERIS_ENV : &'static str = "EPHEMERIS_DIR";
pub const EPHEMERIS_DIRNAME : &'static str = ".ephemeris";

const EPH_PROJECTNAME : &'static str = "projects.toml";
const EPH_TASKNAME : &'static str = "tasks.toml";


