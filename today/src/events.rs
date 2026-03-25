use chrono::{NaiveDate, Datelike};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MonthDay {
    month: u32,
    day: u32,
}
impl MonthDay {
    pub fn new(month: u32, day: u32) -> Self {
        Self { month, day }
    }
    pub fn from_string(s: &str) -> Self {
        assert!(s.len() == 4);
        let month_str = &s[..2];
        let month = month_str.parse().unwrap();
        let day: u32 = s[2..].parse().unwrap();
        Self { month, day }
    }
    pub fn month(&self) -> u32 { self.month }
    pub fn day(&self) -> u32 { self.day }
}       

#[derive(Debug)]
pub enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
pub struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}
impl Event {
    pub fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Self { 
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }
    fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }
    pub fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => 
                MonthDay {month: date.month(), day: date.day()},    
        }
    }
    pub fn description(&self) -> String {
        self.description.clone()
    }
    pub fn category(&self) -> Category {
        self.category.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}
impl Category {
    pub fn new(primary: &str, secondary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: Some(secondary.to_string()),
        }
    }
    pub fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }
    pub fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() < 2 {
            Self::from_primary(s)
        } else {
            Self::new(parts[0], parts[1])
        }
    }
    pub fn primary(&self) -> String {
        self.primary.clone()
    }
    pub fn secondary(&self) -> Option<String> {
        self.secondary.clone()
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
            match &self.kind {
                EventKind::Singular(date) => date.year(),
            },
        self.description, self.category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_both_individually() {
        let category = Category::new("primary", "secondary");
        assert_eq!(category.primary, "primary");
        assert_eq!(category.secondary, Some("secondary".to_string()));
    }
    #[test]
    fn test_primary_only() {
        let category = Category::from_primary("primary");
        assert_eq!(category.primary, "primary");
        assert_eq!(category.secondary, None);
    }
    #[test]
    fn test_both_str() {
        let category = Category::from_str("primary/secondary");
        assert_eq!(category.primary, "primary");
        assert_eq!(category.secondary, Some("secondary".to_string()));
    }
}