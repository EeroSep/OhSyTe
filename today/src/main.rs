use std::path::PathBuf;
use std::fs;
use dirs;
use clap::{Parser, Subcommand};
use chrono::{NaiveDate, Datelike, Local};

use today::{Config, add_event, create_providers};
use today::events::{MonthDay, Category, Event};
use today::filters::{FilterBuilder};
use today::birthday::handle_birthday;

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// List add event providers
    Providers,
    /// Adds an event to an event provider
    Add {
        #[arg(short, long, help = "Provider name to add the event to")]
        provider_name: String,
        #[arg(short, long, help = "Event date in YYYY-MM-DD format")]
        date: String,
        #[arg(short = 'e', long, help = "Event description")]
        description: String,
        #[arg(short, long, help = "Event category")]
        category: String,
    },
    
}

#[derive(Parser)]
#[command (name = "today")]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,
    #[arg(short, long, help = "Exclude categories, comma-seperated (like a/b,c/d)")]
    exclude: Option<String>,
    #[arg(short, long = "no-birthday", help = "No age calculation or birthday message")]
    no_birthday: bool,
}


fn main() {
    let args = Args::parse(); 

    let month_day = if let Some(md) = args.date {
        MonthDay::from_string(&md)
    } else {
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };

    let mut fb = FilterBuilder::new();
    fb = fb.month_day(month_day);

    if let Some(exclude_str) = args.exclude {
        let exclude_categories: Vec<Category> = exclude_str.split(",").map(|s| Category::from_str(s)).collect();
        fb = fb.exclude_category(exclude_categories);
    }
    let filter = fb.build();

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
            let config: Config = toml::from_str(&config_str).expect("valid configuration file");
            match args.cmd {
                Some(Command::Providers) => {
                    let providers = create_providers(&config, &path);
                    println!("Event providers (adding supported): ");
                    for provider in providers {
                        println!("{} {} ({})", provider.name(), provider.kind(),
                        if provider.is_add_supported() { "*" } else { " " });
                    }
                },
                Some (Command::Add { provider_name, date, description, category }) => {
                    let category = Category::from_str(&category);
                    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
                    let event = Event::new_singular(date, description, category);
                    add_event(&config, &path, &provider_name, &event);
                },
                _ => {
                    if !args.no_birthday {
                        handle_birthday();
                    }
                    if let Err(e) = today::run(&config, &path, &filter) {
                        eprintln!("Error: {}", e);
                        return;
                    }
                },
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