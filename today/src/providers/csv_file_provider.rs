use std::path::{Path, PathBuf};
use chrono::NaiveDate;
use csv::{ReaderBuilder, Writer};
use crate::EventProvider;
use crate::events::{Event, Category, EventKind};
use crate::filters::EventFilter;
use crate::providers::AddEventError;
use std::io::BufWriter;


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
    fn kind(&self) -> String {
        "CSV".to_string()
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
    fn is_add_supported(&self) -> bool {
        true
    }
    fn add_event(&self, event: &Event) -> Result<(), AddEventError> {
        let file = std::fs::OpenOptions::new()
            .append(true)
            .open(self.path.clone())
            .expect("path to csv file");
        let writer = BufWriter::new(file);
        let mut csv_writer = Writer::from_writer(writer);

        #[allow(unreachable_patterns)] // This is for no warnings, because there is only singular events in the eventkind enum
        let date_string = match event.kind {
            EventKind::Singular(date) => 
            date.format("%Y-%m-%d").to_string(),
            _ => return Err(super::AddEventError::Failed("Failed to add event".to_string())),
        };

        csv_writer.write_record([
            date_string,
            event.description().to_string(),
            event.category().to_string()
        ]).unwrap();
        csv_writer.flush().unwrap();
        Ok(())
            
        
    }
}
 