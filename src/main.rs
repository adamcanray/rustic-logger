extern crate chrono;

use std::io::prelude::*;
use std::fs::{File,OpenOptions};
use std::io;
use chrono::{DateTime,Local};


fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = r#try!(OpenOptions::new().
                        append(true).
                        write(true).
                        create(true).
                        open(filename));
    r#try!(file.write_all(bytes));
    Ok(())
}

fn log_time(filename: &'static str) -> io::Result<()> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    r#try!(record_entry_in_log(filename, &bytes));
    Ok(())
}

fn main() {
    match log_time("log.txt") {
        Ok(..) => println!("File created!"),
        Err(e) => println!("Error: {}", e)
    }
}