
use crate::*;

pub fn ephemeris_state_dir() -> Result<String, String> {
    match std::env::var_os(EPHEMERIS_ENV) {
        Some(v) => Ok(v.into_string().unwrap()),
        None => {
            match dirs::home_dir() {
                Some(h) => Ok(format!("{}/{}", h.into_os_string().into_string().unwrap(), EPHEMERIS_DIRNAME)),
                None => return Err(String::from("Unable to locate home directory.")),
            }
        },
    }
}
