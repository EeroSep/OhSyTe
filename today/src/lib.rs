pub mod birthday;
pub mod events;
mod providers;
pub mod filters;

use std::error::Error;
use events::{Event};
use crate::providers::{EventProvider, TestProvider, TextFileProvider, CsvFileProvider, SQLiteProvider, WebProvider};
use std::path::Path;
use serde::Deserialize;
use filters::{ EventFilter };

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    resource: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}

pub fn create_providers(config: &Config, config_path: &Path) -> Vec<Box<dyn EventProvider>> {
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
            "sqlite" => {providers.push(Box::new(
                SQLiteProvider::new(&cfg.name, &path)))
            },
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            },
            _ => {eprintln!("Unable to make provider: {:?}", cfg);
            },
        }
    }
    let test_provider = TestProvider::new("Test provider");
    providers.push(Box::new(test_provider));

    providers
}

pub fn add_event(config: &Config, config_path: &Path, provider_name: &str, event: &Event) {
    let providers = create_providers(config, config_path);
    let mut provider: Option<&dyn EventProvider> = None;
    for p in &providers {
        if p.name() == provider_name {
            provider = Some(p.as_ref());
            break;
        }
    }
    match provider {
        Some(p) => {
            if p.is_add_supported() {
                let _ = p.add_event(event);
            } else {
                eprintln!("Provider '{}' does not support adding events", provider_name);
            }
        },
        None => eprintln!("Provider '{}' not found", provider_name),
    }
}

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter) -> Result<(), Box<dyn Error>> {
    
    let mut events: Vec<Event> = Vec::new();
    let providers = create_providers(config, config_path);
    let mut count = 0;
    
    for provider in providers{
        provider.get_events(&filter, &mut events);
        let new_count = events.len();
        println!("Got {} events from provider {}", new_count - count, provider.name());
        count = new_count;
    }
    for event in events {
        println!("{}", event);
    }
    Ok(())
}