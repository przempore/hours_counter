use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DayData {
    hours: u32,
    minutes: u32,
}

impl DayData {
    /// Convert hours and minutes to decimal hours
    pub fn to_hours(&self) -> f32 {
        self.hours as f32 + (self.minutes as f32 / 60.0)
    }

    /// Create DayData from decimal hours
    pub fn from_hours(hours: f32) -> Self {
        let total_minutes = (hours * 60.0) as u32;
        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    /// Format time as HH:MM
    pub fn format(&self) -> String {
        format!("{}:{:02}", self.hours, self.minutes)
    }

    /// Create new DayData instance
    pub fn new(hours: u32, minutes: u32) -> Self {
        Self { hours, minutes }
    }
}

