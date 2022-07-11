
#[allow(unused_imports)]
use tz::{DateTime, TimeZone, UtcDateTime};
use tzdb::{time_zone, tz_by_name};
use ephemeris::state::State;
use crate::*;


fn cmd_time_local() {

    let tz_local = match TimeZone::local() {
        Ok(tz) => tz,
        Err(e) => {
            println!("Unable to deduce timezone.");
            println!("{}", e);
            return;
        }
    };
    let tz_info = tz_local.find_current_local_time_type().unwrap();

    let tzname = tz_info.time_zone_designation();
    let tzdst = tz_info.is_dst();
    println!("Timezone: {}", tzname);
    println!("DST: {}", tzdst);
}

fn cmd_time_test() {
	println!("Time!");

    let now = DateTime::now(tz_by_name("Europe/Zurich").unwrap()).unwrap();
    let tz = time_zone::america::SANTIAGO;
    let tsant = now.project(tz).unwrap();

    println!("Time Europe/Zurich now is {}", now);
    println!("Time America/Santiago now is {}", tsant);



}

pub fn cmd_time(_state: &mut Box<State>, cmd: &crate::Time) {
    match &cmd.subcmd {
        TimeSubCommand::Local(_l) => {
            cmd_time_local()
        }
        TimeSubCommand::Test(_t) => {
       		cmd_time_test()
       	}
    }
}
