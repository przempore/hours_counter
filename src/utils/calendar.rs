use time::{Date, OffsetDateTime, Month, Weekday};

pub struct Calendar {

}

impl Calendar {
    pub const WEEKDAYS_NAMES: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    pub const MONTHS_NAMES: [&str; 12] = [
        "January", "February", "March", "April", "May", "June", "July", "August", "September",
        "October", "November", "December",
    ];

    pub fn today() -> Date {
        OffsetDateTime::now_utc().date()
    }

    pub fn is_weekend(year: i32, month: Month, day: u8) -> bool {
    if let Some(date) = Date::from_calendar_date(year, month, day).ok() {
        matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
    } else {
        false
    }
}
}
