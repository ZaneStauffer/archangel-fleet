
use time::*;
use time::{Date, PrimitiveDateTime, OffsetDateTime, UtcOffset};
use time::Weekday::Wednesday;
use time::macros::{date, datetime};
use colored::*;
use std::result::Result;

#[derive(Debug)]
pub enum Error{
    LoggingError
}

pub enum AlertType{
    DEFAULT,
    ALERT,
    WARNING
}

pub fn get_alert(alert_type: &AlertType) -> colored::Color{
    match alert_type{
        AlertType::ALERT => {
            colored::Color::Yellow
        },
        AlertType::WARNING => {
            colored::Color::Red
        },
        _ => {
            colored::Color::White
        }
    }
}

pub fn timestamp() -> String{
    let time = time::OffsetDateTime::now_utc();
    format!("[{}]", time)
}
// i am coding..... (:
    // i am an EVIL hacker....
        // they call me anonymous
            // THE anon
pub fn log(message: &String, alert_type: AlertType, should_save: bool){
    let input_char = "!>".white();
    // Log Format
    // [timestamp (alert)] !> message
    // if save=true save to log file [do later]
    let log = format!("{} {} {}", timestamp().as_str().color(get_alert(&alert_type)), input_char, message);
    if should_save {
        // Appends this log to the logfile
        save(&log).unwrap();
    }
    println!("{}", log);
}

pub fn system_log(message: &String, should_save: bool){
    let log = format!(">> {} <<", message.cyan());
    if should_save {
        // Appends this log to the logfile
        save(&log).unwrap();
    }
    println!(">> {} <<", message.cyan())
}

// takes the finished log as input
fn save(log: &String) -> Result<(), Error>{
    // Append the log to the current logfile, return result
    Ok(())
}