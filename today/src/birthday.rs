use std::env;
use chrono::{NaiveDate, Datelike, Local};


pub fn handle_birthday() {
    const NAME: &str = "BIRTHDAY";
    let value = env::var(NAME);
    if !value.is_ok() {
        return;
    }
    let value = value.unwrap();
    match NaiveDate::parse_from_str(&value, "%F") {
        Ok(birthdate) => {
            let mut result = String::new();

            let today: NaiveDate = Local::now().date_naive();
            if birthdate.month() == today.month() && birthdate.day() == today.day() {
                result.push_str("Happy birthday! ");
            }
            let diff = today.signed_duration_since(birthdate);
            let day_count = diff.num_days();

            let message = make_message(day_count);
            result.push_str(&message);
            println!("{}", result);
    },
    Err(_) => {
        eprintln!("Error in the '{}' environment variable: \
            '{}' is not a valid date.", NAME, value);
        }
    }
}
fn make_message(day_count: i64) -> String {
    let mut message = String::new();
    if day_count > 0 {
        message.push_str(&format!("You are {} days old.", day_count));
        if day_count % 1000 == 0 && day_count != 0 {
            message.push_str(" That's a nice round number!");
        }
    }else if day_count < 0 {
        message.push_str("Are you from the future?");
    }else if day_count == 0 {
        message.push_str("Looks like you're new here.");
    }
    message
}
