use std::path::{Path, PathBuf};
use chrono::NaiveDate;
use crate::events::{Event, Category};
use crate::EventProvider;
use sqlite::{Connection, State};
use std::collections::HashMap;
use crate::filters::EventFilter;

pub struct SQLiteProvider {
    name: String,
    path: PathBuf,
}

impl SQLiteProvider {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
        }
    }
    pub fn get_categories(&self, connection: &Connection) -> HashMap<i64, Category> {
        let mut category_map: HashMap<i64, Category> = HashMap::new();
        let category_query = "SELECT category_id, primary_name, secondary_name FROM category";
        let mut statement = connection.prepare(category_query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let category_id = statement.read::<i64, _>("category_id").unwrap();
            let primary = statement.read::<String, _>("primary_name").unwrap();
            let secondary = statement.read::<Option<String>, _>("secondary_name").unwrap();
            let category = match secondary {
                Some(sec) => Category::new(&primary, &sec),
                None => Category::from_primary(&primary),
            };
            category_map.insert(category_id, category);
        }
        category_map
    }

}

fn make_date_part(filter: &EventFilter) -> String {
    if let Some(month_day) = filter.month_day() {
        format!("strftime('%m-%d', event_date) = '{:02}-{:02}'",
        month_day.month(), month_day.day())
    } else {
        "".to_string()
    }
}

fn make_category_part(filter: &EventFilter, category_map: &HashMap<i64, Category>) -> String {
    if let Some(filter_category) = filter.category() {
        let mut filter_category_id: Option<i64> = None;
        for (id, cat) in category_map {
            if *cat == filter_category {
                filter_category_id = Some(*id);
                break;
            }
        }
        match filter_category_id {
            Some(id) => format!("category_id = {}", id),
            None => "1=0".to_string(),
        }
    } else {
        "1=0".to_string()
    }
}

fn make_text_part(filter: &EventFilter) -> String {
    if let Some(text) = filter.text() {
        format!("event_description LIKE '%{}%'", text)
    } else {
        "".to_string()
    }
}

fn make_query(filter: &EventFilter, category_map: &HashMap<i64, Category>) -> String {
    let base_query = "SELECT event_date, event_description, category_id FROM event";
    let mut parts: Vec<String> = Vec::new();
    if filter.contains_month_day() {
        parts.push(make_date_part(filter));
    }
    if filter.contains_category() {
        parts.push(make_category_part(filter, category_map));
    }
    if filter.contains_text() {
        parts.push(make_text_part(filter));
    }
    let mut result = "SELECT event_date, event_description, category_id FROM event".to_string();
    if !parts.is_empty() {
        result.push_str(" WHERE ");
        result.push_str(&parts.join(" AND "));
    }
    result
}

impl EventProvider for SQLiteProvider {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let connection = Connection::open(self.path.clone()).unwrap();
        let category_map = self.get_categories(&connection);
        let event_query = make_query(filter, &category_map);
        let mut statement = connection.prepare(event_query).unwrap();
        while let Ok(State::Row) = statement.next() {
            let date_string = statement.read::<String, _>("event_date").unwrap();
            let date = NaiveDate::parse_from_str(&date_string, "%F").unwrap();
            let description = statement.read::<String, _>("event_description").unwrap();
            let category_id = statement.read::<i64, _>("category_id").unwrap();
            let category = category_map.get(&category_id).unwrap();
            let event = Event::new_singular(date, description.to_string(), category.clone());
            if filter.accepts(&event) {
                events.push(event);
            }
        }
    }
}