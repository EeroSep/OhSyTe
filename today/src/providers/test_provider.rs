use crate::events::{Event, Category};
use chrono::{NaiveDate, Local};
use crate::EventProvider;
use crate::filters::EventFilter;
use crate::providers::AddEventError;

pub struct TestProvider {
    name: String,
}
impl TestProvider {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
impl EventProvider for TestProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();
        let test_event = Event::new_singular(
        today, 
        String::from("Test event for today"), 
        Category::from_primary("test")
        );
        if filter.accepts(&test_event) {
            events.push(test_event);
        }
    }
    fn is_add_supported(&self) -> bool {
        false
    }
    fn add_event(&self, _event: &Event) -> Result<(), AddEventError> {
        Err(AddEventError::NotSupported)
    }
}
