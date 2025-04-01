use time::{Date, Month, OffsetDateTime, Weekday};

#[derive(Debug, Clone)]
pub struct Calendar {
    pub date: Date,
}

impl Calendar {
    pub fn from(year: i32, month: Month, day: u8) -> Self {
        Self {
            date: Date::from_calendar_date(year, month, day).expect("correct date format"),
        }
    }

    pub fn today() -> Self {
        Self {
            date: OffsetDateTime::now_local()
                .expect("correct local time")
                .date(),
        }
    }

    pub fn weekdays_names() -> [String; 7] {
        [
            Weekday::Monday.to_string(),
            Weekday::Tuesday.to_string(),
            Weekday::Wednesday.to_string(),
            Weekday::Thursday.to_string(),
            Weekday::Friday.to_string(),
            Weekday::Saturday.to_string(),
            Weekday::Sunday.to_string(),
        ]
    }

    pub fn day(&self) -> u8 {
        self.date.day()
    }

    pub fn month(&self) -> Month {
        self.date.month()
    }

    pub fn weekday(&self) -> Weekday {
        self.date.weekday()
    }

    pub fn year(&self) -> i32 {
        self.date.year()
    }

    pub fn is_weekend(year: i32, month: Month, day: u8) -> bool {
        let Some(date) = Date::from_calendar_date(year, month, day).ok() else {
            return false;
        };

        matches!(date.weekday(), Weekday::Saturday | Weekday::Sunday)
    }

    pub fn last_visible_days(&self, date: &Self) -> (u8, u8) {
        let current_month = Self::from(date.year(), date.month(), 1);
        let first_visible = self.date.month().length(self.date.year())
            - current_month.weekday().number_from_monday()
            + 2;

        (first_visible, self.month().length(self.year()))
    }
}
