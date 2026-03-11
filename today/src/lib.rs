mod birthday;
mod events;
mod providers;

use std::error::Error;
use chrono::{NaiveDate, Local, Datelike};
use birthday::handle_birthday;
use events::{Event, Category, MonthDay};
use crate::providers::{EventProvider, TestProvider, TextFileProvider, CsvFileProvider};
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    name: String,
    kind: String,
    resource: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    providers: Vec<ProviderConfig>,
}

fn create_providers(config: &Config, config_path: &Path) -> Vec<Box<dyn EventProvider>> {
    let mut providers: Vec<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            "text" => {providers.push(Box::new(
                TextFileProvider::new(&cfg.name, &path)))
            },
            "csv" => {providers.push(Box::new(
                CsvFileProvider::new(&cfg.name, &path)))
            },
            _ => {eprintln!("Unable to make provider: {:?}", cfg);
            },
        }
    }
    let test_provider = TestProvider::new("Test provider");
    providers.push(Box::new(test_provider));

    providers
}

pub fn run(config: &Config, config_path: &Path) -> Result<(), Box<dyn Error>> {
    handle_birthday();
    
    let mut events: Vec<Event> = Vec::new();
    let providers = create_providers(config, config_path);
    for provider in providers{
        provider.get_events(&mut events);
    }

    //let today_month_day = MonthDay::new(Local::now().month(), Local::now().day());
    let today_month_day = MonthDay::new(1, 26);
    for event in events {
        if event.month_day() == today_month_day {
            println!("{}", event);
        }
    }
    Ok(())
}