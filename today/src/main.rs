use chrono::{NaiveDate, Datelike};
use std::fmt;

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: u8,
    day: u8,
}
impl MonthDay {
    fn new(month: u8, day: u8) -> Self {
        Self { month, day }
    }
    fn from_string(s: &str) -> Self {
        assert!(s.len() == 4)
        let month = s[0..2].parse().unwrap();
        let day = s[2..].parse().unwrap();
        Self { month, day }
}       

#[derive(Debug)]
pub enum EventKind {
    Singluar(NaiveDate),
}

#[derive(Debug)]
pub struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}
impl Event {
    pub fn new_singluar(date: NaiveDate, description: String, category: String) -> Self {
        Self { 
            kind: EventKind::Singluar(date),
            description,
            category,
        }
    }
    pub fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singluar(date) => date.year(),
        }
    }
    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singluar(date) => 
                MonthDay {month: date.month(), day: date.day()},    
        }
    }
}

#[derive(Debug)]
struct Category {
    primary: String,
    secondary: Option<String>,
}
impl Category {
    fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }
    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() < 2 {
            Self::from_primary(s)
        } else {
            Self::new(parts[0], parts[1])
        }
    }
}
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} ({})",
            self.date.year, self.description, self.category)
    }
}

fn main() {


}
#[cfg(test)]
mod tests {
    use crate::{Category}
    #[test]
    fn test_both_individually() {

    }
    fn test_both_str() {

    }
    fn test_primary_only() {
    }
}