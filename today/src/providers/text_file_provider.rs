use crate::events::{Event, Category};
use crate::EventProvider;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead};

enum ReadingState {
    Date,
    Description,
    Category,
    Separator,
}

pub struct TextFileProvider {
    name: String,
    path: PathBuf,
}

impl TextFileProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self { 
            name: name.to_string(), 
            path: path.to_path_buf() 
        }
    }
}

impl EventProvider for TextFileProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, events: &mut Vec<Event>) {
        let f = File::open(self.path.clone()).expect("path to text file");
        let reader = BufReader::new(f);
        let mut state = ReadingState::Date;
        let mut date_string = String::new();
        let mut description = String::new();
        let mut category_string = String::new();
        for line_result in reader.lines() {
            let line = line_result.expect("read line");
            match state {
                ReadingState::Date => {
                    date_string = line;
                    state = ReadingState::Description;
                },
                ReadingState::Description => {
                    description = line;
                    state = ReadingState::Category;
                },
                ReadingState::Category => {
                    category_string = line;
                    state = ReadingState::Separator;
                },
                ReadingState::Separator => {
                    match chrono::NaiveDate::parse_from_str(&date_string, "%F") {
                        Ok(date) => {
                            let category = Category::from_str(&category_string);
                            let event = Event::new_singular(date, description.clone(), category);
                            events.push(event);
                        },
                        Err(_) => {
                            eprintln!("Invalid timestamp '{}'", date_string);
                        }
                    }
                    state = ReadingState::Date;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    #[test]
    fn test_text_file_provider() {
        let test_path = Path::new("test_events.txt");
        let mut file = File::create(test_path).expect("create test file");
        writeln!(file, "2026-02-28").expect("write date");
        writeln!(file, "This day").expect("write description");
        writeln!(file, "test").expect("write category");
        writeln!(file, " ").expect("write separator"); 

        let provider = TextFileProvider::new("Test Text Provider", test_path);
        let mut events: Vec<Event> = Vec::new();
        provider.get_events(&mut events);

        assert_eq!(events.len(), 1);
        assert_eq!(format!("{}", events[0]), "2026: This day (test)");
        std::fs::remove_file(test_path).expect("remove test file");
    }

}