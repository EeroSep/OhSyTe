use crate::events::{Event, Category};
use chrono::{NaiveDate, Local};
use crate::EventProvider;

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
    fn get_events(&self, events: &mut Vec<Event>) {
        let today: NaiveDate = Local::now().date_naive();
        let test_event = Event::new_singular(
        today, 
        String::from("Test event for today"), 
        Category::from_primary("test")
        );
        events.push(test_event);
    }
}
