use std::{fs::OpenOptions, io::{self, Write}};

use chrono::{DateTime, Local};

pub enum TSeverity {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

pub fn log(filename: &'static str, entry: &'static str, severity: Option<TSeverity>, time_format: Option<&'static str>) -> io::Result<String> {
    let entry_time = formatted_time_entry(time_format);
    
    let entry_severity = match severity {
        Some(TSeverity::DEBUG) => "DEBUG ",
        Some(TSeverity::INFO) => "INFO  ",
        Some(TSeverity::WARN) => "WARN  ",
        Some(TSeverity::ERROR) => "ERROR ",
        None => "",
    };
    
    {
        let together = format!("{}{} {}\n", entry_severity, entry_time, entry);
        let entry_bytes = together.as_bytes();
        record_entry_in_log(filename, &entry_bytes)?;
    }
    
    Ok(entry_time)
}

fn formatted_time_entry(time_format: Option<&'static str>) -> String {
    let default_time_format = "%Y-%m-%d %H:%M:%S,%3f";
    let time_format_to_use = time_format.unwrap_or(default_time_format);
    
    let local: DateTime<Local> = Local::now();
    let formatted = format!("[{}]", local.format(time_format_to_use).to_string());
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().
                        append(true).
                        write(true).
                        create(true).
                        open(filename)?;
    file.write_all(bytes)?;
    Ok(())
}