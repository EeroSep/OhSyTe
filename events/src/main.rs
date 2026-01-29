struct Date {
    year: i16,
    month: Month,
    day: u8,
}
impl Date {
    fn new(year: i16, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, PartialEq)]
struct MonthDay {
    month: Month,
    day: u8,
}
impl MonthDay {
    fn new(month: Month, day: u8) -> Self {
        Self { month, day }
    }
}

struct Event {
    date: Date,
    description: String,
    category: Category,
}
impl Event {
    fn new(date: Date, description: String, category: Category) -> Self {
        Self { date, description, category }
    }
    fn month_day(&self) -> MonthDay {
        MonthDay {
            month: self.date.month,
            day: self.date.day,
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
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn day_count(month: Month, year: i32) -> u8 {
    match month {
        Month::April | Month::June | Month::September | Month::November => 30,
        Month::February => { if is_leap_year(year) { 29 } else { 28 } },
        _ => 31,
    }
}

fn main() {
    let events = [
        Event::new(Date::new(1996, Month::January, 23), 
        "JDK 1.0 released".to_string(), 
        Category::new("Technology", "Java")),

        Event::new(Date::new(2008, Month::December, 3), 
        "Python 3.0 released".to_string(), 
        Category::new("Technology", "Python")),

        Event::new(Date::new(2015, Month::May, 15), 
        "Rust 1.0.0 released".to_string(), 
        Category::new("Technology", "Rust")),

        Event::new(Date::new(2025, Month::September, 16), 
        "Java 25 released".to_string(), 
        Category::new("Technology", "Java")),

        Event::new(Date::new(2025, Month::October, 7), 
        "Python 3.14 released".to_string(), 
        Category::new("Technology", "Python")),

        Event::new(Date::new(2025, Month::December, 11), 
        "Rust 1.92.0 released".to_string(), 
        Category::new("Technology", "Rust")),

        Event::new(Date::new(2026, Month::January, 15), 
        "Rising tension over Greenland".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 16), 
        "USA invades Greenland".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 16), 
        "NATO joins the war".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 17), 
        "EU dumps US bonds".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 17), 
        "dollar crashes".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 18), 
        "World war III begins".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 19), 
        "China invades Taiwan".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 19), 
        "Russia atacks the US".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 20), 
        "Greenland peace talks fail".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 21), 
        "Global markets collapse".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(2026, Month::January, 22), 
        "USA launches nukes".to_string(), 
        Category::from_primary("Politics")),

        Event::new(Date::new(1905, Month::January, 22), 
        "Russian revolution".to_string(), 
        Category::new("History", "Politics")),

        Event::new(Date::new(1978, Month::January, 23), 
        "Sweden bans aerosol sprays".to_string(), 
        Category::new("History", "Environment")),

        Event::new(Date::new(1984, Month::January, 24), 
        "Apple Macintosh unveiled".to_string(),
        Category::new("Technology", "History")),

        Event::new(Date::new(1971, Month::January, 25), 
        "Military coup in Uganda".to_string(),
        Category::new("Politics", "History")),

        Event::new(Date::new(1340, Month::January, 26),
         "Edward III proclaimed King of France".to_string(),
         Category::new("Politics", "History")),

        Event::new(Date::new(1820, Month::January, 27), 
         "Russia discovers Antarctica".to_string(),
         Category::new("Politics", "History")),

        Event::new(Date::new(1813, Month::January, 28),
         "Pride and Prejudice is published".to_string(),
         Category::new("Literature", "History")),

        Event::new(Date::new(1595, Month::January, 29),
         "Romeo and Juliet first performed".to_string(),
         Category::from_primary("History")),
    ];

    for day in 15..=29 {
        let month_date = MonthDay::new(Month::January, day);
        println!("\nEvents of {}.{}.", day, 1); 
        let mut any_luck = false;
        for event in &events {
            if event.month_day() == month_date {
                let categories = match &event.category.secondary {
                    Some(secondary) => &format!("{}/{}", 
                    &event.category.primary, secondary),
                    None => &format!("{}", &event.category.primary)
                };
                println!("{}: Event: {}, Category: {}", 
                event.date.year, event.description, categories);
                any_luck = true;
            }   
        }
        if !any_luck {
            println!("No events on {}.1", day)
        }
    }
}
