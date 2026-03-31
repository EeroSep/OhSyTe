use std::path::PathBuf;
use std::fs;
use dirs;
use clap::Parser;
use chrono::{NaiveDate, Datelike, Local};

use today::Config;
use today::events::MonthDay;
use today::filters::{FilterBuilder};

#[derive(Parser)]
#[command (name = "today")]
struct Args {
    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,
}

fn main() {
    let args = Args::parse(); 

    let month_day = if let Some(md) = args.date {
        MonthDay::from_string(&md)
    } else {
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };
    //let category = Category::from_str("test");
    let filter = FilterBuilder::new()
    .month_day(month_day)
    //.category(Category::from_str("test"))
    .build();

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
            let config: Config = toml::from_str(&config_str).expect("valid configuration file");
            if let Err(e) = today::run(&config, &path, &filter) {
                eprintln!("Error: {}", e);
                return;
            }
        },
        None => {
            eprintln!("Unable to configure the application");
            return;
        }
    }
}

fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        println!("Config directory: '{}'", config_dir.display());

        // Check if our config directory exists
        let config_path = config_dir.join(app_name);
        print!("App config directory: '{}'", config_path.display());

        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            } else {
                print!(" - created");
            }
        } else {
            print!(" - exists");
        }
        println!();

        return Some(config_path);
    }

    None
}