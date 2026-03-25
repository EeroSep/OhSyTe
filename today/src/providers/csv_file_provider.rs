use std::path::{Path, PathBuf};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use crate::EventProvider;
use crate::events::{Event, Category};
use crate::filters::EventFilter;

pub struct CsvFileProvider {
    name: String,
    path: PathBuf,
}
impl CsvFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
}
impl EventProvider for CsvFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(self.path.clone())
            .expect("path to csv file");
        for result in reader.records() {
            let record = result.unwrap();
            let date_string = record[0].to_string();
            let description = record[1].to_string();
            let category_string = record[2].to_string();
            match NaiveDate::parse_from_str(&date_string, "%F") {
                Ok(date) => {
                    let category = Category::from_str(&category_string);
                    let event = Event::new_singular(date, description.clone(), category);
                    if filter.accepts(&event) {
                        events.push(event);
                    }
                },
                Err(_) => {
                    eprintln!("Invalid date '{}'", date_string);
                }
            }
        }
    }   
}
 