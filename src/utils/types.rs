use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DayData {
    pub day: u8,
    pub is_current_month: bool,
    pub is_today: bool,
    pub is_weekend: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CalendarData {
    pub year: i32,
    pub month: u8,
    pub month_name: String,
    pub days: Vec<DayData>,
    pub weekdays: [String; 7],
}
