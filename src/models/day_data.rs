use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DayData {
    hours: u32,
    minutes: u32,
}

impl DayData {
    /// Create new DayData instance.
    pub fn new(hours: u32, minutes: u32) -> Self {
        // Ensure minutes are capped at 59
        let total_minutes = hours * 60 + minutes;
        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    /// Get hours component.
    pub fn hours(&self) -> u32 {
        self.hours
    }

    /// Get minutes component.
    pub fn minutes(&self) -> u32 {
        self.minutes
    }

    /// Convert hours and minutes to decimal hours.
    pub fn to_f32_hours(&self) -> f32 {
        self.hours as f32 + (self.minutes as f32 / 60.0)
    }

    /// Create DayData from decimal hours.
    pub fn from_f32_hours(hours: f32) -> Self {
        if hours < 0.0 {
            return Self::default();
        }
        let total_minutes = (hours * 60.0).round() as u32;
        Self {
            hours: total_minutes / 60,
            minutes: total_minutes % 60,
        }
    }

    /// Format time as HH:MM.
    pub fn format(&self) -> String {
        format!("{}:{:02}", self.hours, self.minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_day_data() {
        let data = DayData::new(8, 30);
        assert_eq!(data.hours, 8);
        assert_eq!(data.minutes, 30);
    }

    #[test]
    fn test_new_day_data_overflow() {
        let data = DayData::new(1, 75);
        assert_eq!(data.hours, 2);
        assert_eq!(data.minutes, 15);
    }

    #[test]
    fn test_to_f32_hours() {
        let data = DayData::new(8, 30);
        assert_eq!(data.to_f32_hours(), 8.5);
        let data = DayData::new(2, 15);
        assert_eq!(data.to_f32_hours(), 2.25);
    }

    #[test]
    fn test_from_f32_hours() {
        let data = DayData::from_f32_hours(8.5);
        assert_eq!(data, DayData::new(8, 30));
         let data = DayData::from_f32_hours(2.25);
        assert_eq!(data, DayData::new(2, 15));
        let data = DayData::from_f32_hours(0.0);
        assert_eq!(data, DayData::new(0, 0));
         let data = DayData::from_f32_hours(-1.0);
        assert_eq!(data, DayData::new(0, 0));
    }

    #[test]
    fn test_format() {
        let data = DayData::new(8, 30);
        assert_eq!(data.format(), "8:30");
        let data = DayData::new(9, 5);
        assert_eq!(data.format(), "9:05");
    }
}
