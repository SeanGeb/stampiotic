use std::env::args;
use std::fs::metadata;
use std::process::exit;
use std::time::SystemTime;

use humantime::{format_duration, parse_duration};

fn main() -> ! {
    if let Some(err) = run() {
        println!("{err}");
        exit(1)
    } else {
        exit(0)
    }
}

fn run() -> Option<String> {
    if args().len() < 3 {
        Some(format!(concat!(
            "stampiotic\n",
            "Usage: stampiotic <stampfile> <timeout>\n",
            "Examples:\n",
            "  stampiotic /run/foo/foo-server.stamp 60s\n",
            "  stampiotic bar.stamp 30\n",
            "  stampiotic slow-process.stamp 1d\n",
            "Exits with a status code of 1 if the stampfile doesn't exist, or is too old.\n",
        )))
    } else {
        let max_age = args().nth(1).unwrap();
        let stampfiles = args().skip(2);

        let max_age = match parse_duration(&max_age) {
            Ok(x) => x,
            Err(_) => return Some(format!("unable to parse duration {max_age:?}")),
        };

        for stampfile in stampfiles {
            let metadata = match metadata(&stampfile) {
                Ok(m) => m,
                Err(e) => return Some(format!("unable to read stampfile {stampfile:?}: {e}")),
            };

            let modified = match metadata.modified() {
                Ok(m) => m,
                Err(e) => return Some(format!("unable to get mtime: {e}")),
            };

            let stampfile_age = match SystemTime::now().duration_since(modified) {
                Ok(s) => s,
                Err(e) => return Some(format!("unable to determine age: {e}")),
            };

            if stampfile_age > max_age {
                return Some(format!(
                    "File {stampfile:?} is too old ({} > {})",
                    format_duration(stampfile_age),
                    format_duration(max_age)
                ));
            }
        }

        None
    }
}
