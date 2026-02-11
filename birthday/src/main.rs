use std::env;
use chrono::{NaiveDate, Datelike};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <birthdate in YYYY-MM-DD format>", args[0]);
        return;
    }
    let birthdate_str = &args[1];
    let birthdate = match NaiveDate::parse_from_str(birthdate_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            eprintln!("Error: Invalid date format or date. Please use YYYY-MM-DD.");
            return;
        }
    };
    //let today = NaiveDate::from_ymd_opt(2026, 1, 28).unwrap();
    let today = chrono::Local::now().date_naive();
    let days_lived = (today - birthdate).num_days();
    if days_lived > 0 {
        println!("You are {} days old.", days_lived);
    }
    if days_lived < 0 {
        println!("Are you from the future?");
    }
    if days_lived % 1000 == 0 && days_lived != 0 {
        println!("That's a nice round number!");
    }
    if today.month() == birthdate.month() && today.day() == birthdate.day() {
        println!("Happy Birthday!");
    }
    if days_lived == 0 {
        println!("Looks like you're new here.");
    }
}
