fn main() {
    let events = [
        (1996_01_23, "JDK 1.0 released"),
        (2008_12_03, "Python 3.0 released"),
        (2015_05_15, "Rust 1.0.0 released"),
        (2025_09_16, "Java 25 released"),
        (2025_10_07, "Python 3.14 released"),
        (2025_12_11, "Rust 1.92.0 released"),
        (2026_01_15, "Rising tension over Greenland"),
        // Hypothetical future events
        (2026_01_16, "USA invades Greenland"),
        (2026_01_16, "NATO joins the war"),
        (2026_01_17, "EU dumps US bonds"),
        (2026_01_17, "dollar crashes"),
        (2026_01_18, "World war III begins"),
        (2026_01_19, "China invades Taiwan"),
        (2026_01_19, "Russia atacks the US"),
        (2026_01_20, "Greenland peace talks fail"),
        (2026_01_21, "Global markets collapse"),
        (2026_01_22, "USA launches nukes"),
    ];
    
    for date in 2026_01_15..=2026_01_22 {
        let date_string = date.to_string();
        let month_string = &date_string[4..6];
        let day_string = &date_string[6..8];
        let day: i32 = day_string.parse().unwrap();
        let month: i32 = month_string.parse().unwrap();
        let month_day = (month, day);
        println!("Events of {}.{}.", day, month);
        let mut any_luck = false;
        for event in events {
            let event_date_string = event.0.to_string();
            let event_month_string = &event_date_string[4..6];
            let event_day_string = &event_date_string[6..8];
            let event_day: i32 = event_day_string.parse().unwrap();
            let event_month: i32 = event_month_string.parse().unwrap();
            let event_month_day = (event_month, event_day);

            let year_string = &event_date_string[0..4];
            let event_year: i32 = year_string.parse().unwrap();
            if event_month_day == month_day {
                println!("{}: {}", event_year, event.1);
                any_luck = true;
            }
        }
        if !any_luck {
            println!("No events found on this date.");
        }
    }
}
