mod birthday;
mod events;
mod providers;

use std::error::Error;
use chrono::{NaiveDate, Local, Datelike};
use birthday::handle_birthday;
use events::{Event, Category, MonthDay};
use crate::providers::{EventProvider, TestProvider, TextFileProvider};
use std::path::Path;

pub fn run() -> Result<(), Box<dyn Error>> {
    handle_birthday();
    
    let mut events: Vec<Event> = Vec::new();
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        Category::new("programming", "rust"),
    ));
    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        Category::new("programming", "rust"),
    ));

    let provider = TestProvider::new("Test provider");
    provider.get_events(&mut events);

    let text_provider = TextFileProvider::new(
        "text events", Path::new("testfile.txt"));

    text_provider.get_events(&mut events);

    let today_month_day = MonthDay::new(Local::now().month(), Local::now().day());
    for event in events {
        if event.month_day() == today_month_day {
            println!("{}", event);
        }
    }
    Ok(())
}