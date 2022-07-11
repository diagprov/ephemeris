
#[allow(unused_imports)]
use chrono::{NaiveDate, NaiveDateTime, FixedOffset, DateTime, Utc, TimeZone};
#[allow(unused_imports)]
use chrono_tz::{Tz, UTC};
use ephemeris::state::State;
use crate::*;


fn cmd_time_test() {
	println!("Time!");

	let dt_utc = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1626958768, 0), Utc);

	let zurich_tz : Tz = "Europe/Zurich".parse().unwrap();
	//let CetTZ : Tz = "CET".parse().unwrap();
	let dt_zrh = zurich_tz.from_local_datetime(&NaiveDateTime::from_timestamp(1626958768, 0));
	//let dt_zrh = CetTZ.from_local_datetime(&NaiveDateTime::from_timestamp(1626958768, 0));

	let tz1: Tz = "Europe/London".parse().unwrap();
	//let tz2: Tz = "CEST".parse().unwrap();
	println!("UTC Time To String {}", dt_utc.to_string());
	println!("UTC Time RFC-2822 {}", dt_utc.to_rfc2822());
	println!("UTC Time RFC-3339 {}", dt_utc.to_rfc3339());

	println!("Zurich Time To String {}", dt_zrh.unwrap().to_string());
	println!("Zurich Time RFC-2822 {}", dt_zrh.unwrap().to_rfc2822());
	println!("Zurich Time RFC-3339 {}", dt_zrh.unwrap().to_rfc3339());

	println!("TZ parsed as Europe/London is {}", tz1);
	//println!("TZ parsed as CEST is {}", tz2);

	println!("Zurich time in UTC Time {}", dt_zrh.unwrap().with_timezone(&Utc));
	println!("UTC time in {} Time {}", zurich_tz, dt_utc.with_timezone(&zurich_tz));

}

pub fn cmd_time(_state: &mut Box<State>, cmd: &crate::Time) {
    match &cmd.subcmd {
        TimeSubCommand::Test(_t) => {
       		cmd_time_test()
       	}
    }
}
