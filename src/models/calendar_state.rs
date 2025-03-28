use super::day_data::DayData;
// Removed LocalResult and Weekday imports below
use chrono::{Datelike, Local, TimeZone};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};


/// Represents a calendar date as (year, month, day)
pub type CalendarDate = (i32, u32, u32);

// Function to get the date for the first specified weekday in a month
fn get_first_weekday_date(year: i32, month: u32, target_weekday: chrono::Weekday) -> Option<u32> {
    (1..=get_days_in_month(year, month)).find(|&day| {
        // Use .single() to get Option<DateTime> and then map
        Local
            .with_ymd_and_hms(year, month, day, 0, 0, 0)
            .single() // Convert LocalResult to Option
            .map(|dt| dt.weekday() == target_weekday)
            .unwrap_or(false) // Now unwrap_or works on Option
    })
}

// Helper to get number of days in month (no changes)
fn get_days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29 // Leap year
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[derive(Clone, Copy)]
pub struct CalendarState {
    // Private signals
    hours_data: RwSignal<HashMap<CalendarDate, DayData>>,
    working_fridays: RwSignal<HashSet<CalendarDate>>,
    pub selected_date: RwSignal<Option<CalendarDate>>,
    show_modal: RwSignal<bool>,
    // Store the date of the first Friday of the currently viewed month
    // This is used to determine which Fridays are "working Fridays" (assumed alternating)
    first_friday_date_in_month: RwSignal<Option<u32>>,
    current_view_month: RwSignal<(i32, u32)>,
}

impl CalendarState {
    /// Initialize calendar state for a specific month and year.
    pub fn new(year: i32, month: u32) -> Self {
        let first_friday = get_first_weekday_date(year, month, chrono::Weekday::Fri);
        Self {
            // Use RwSignal::new()
            hours_data: RwSignal::new(HashMap::new()),
            working_fridays: RwSignal::new(HashSet::new()),
            selected_date: RwSignal::new(None),
            show_modal: RwSignal::new(false),
            first_friday_date_in_month: RwSignal::new(first_friday),
            current_view_month: RwSignal::new((year, month)),
        }
    }

    /// Call this when the viewed month changes.
    pub fn set_view_month(self, year: i32, month: u32) { // Changed to take self by value (Copy)
        self.current_view_month.set((year, month));
        self.first_friday_date_in_month.set(get_first_weekday_date(
            year,
            month,
            chrono::Weekday::Fri,
        ));
    }

    /// Get the currently viewed month and year.
    pub fn current_view_month(self) -> ReadSignal<(i32, u32)> { // Changed to take self by value (Copy)
        self.current_view_month.read_only()
    }

    // --- Modal Management ---

    /// Show the modal for the given date.
    pub fn select_date_and_show_modal(self, date: CalendarDate) { // Changed to take self by value (Copy)
        self.selected_date.set(Some(date));
        self.show_modal.set(true);
    }

    /// Hide the modal.
    pub fn close_modal(self) { // Changed to take self by value (Copy)
        self.selected_date.set(None);
        self.show_modal.set(false);
    }

    /// Get signal indicating if the modal should be shown.
    pub fn is_modal_visible(self) -> ReadSignal<bool> { // Changed to take self by value (Copy)
        self.show_modal.read_only()
    }

    /// Get the currently selected date (if any).
    pub fn get_selected_date(self) -> ReadSignal<Option<CalendarDate>> { // Changed to take self by value (Copy)
        self.selected_date.read_only()
    }

    // --- Hours Data Management ---

    /// Save time for a specific date.
    pub fn save_time_for_selected(self, hours: u32, minutes: u32) { // Changed to take self by value (Copy)
        if let Some(date) = self.selected_date.get() {
            let new_data = DayData::new(hours, minutes);
            let should_update = self
                .hours_data
                .get()
                .get(&date)
                .map_or(true, |existing| existing != &new_data);

            if should_update {
                self.hours_data.update(|data| {
                    data.insert(date, new_data);
                });
            }
        }
        self.close_modal(); // Close modal after saving
    }

    /// Get the hours data for a specific date.
    pub fn get_day_data_for_date(self, date: CalendarDate) -> Signal<Option<DayData>> { // Changed to take self by value (Copy)
         // Use signal()
         let (sig, _) = signal(self.hours_data.get().get(&date).cloned());
         sig.into()
    }

    /// Get the raw hours data map signal (use with caution, prefer specific getters).
    pub fn hours_map(self) -> ReadSignal<HashMap<CalendarDate, DayData>> { // Changed to take self by value (Copy)
        self.hours_data.read_only()
    }

    // --- Working Friday Management ---

    /// Toggle working Friday status for the selected date.
    /// Assumes the selected date IS a Friday.
    pub fn toggle_working_friday_for_selected(self) { // Changed to take self by value (Copy)
        if let Some(date @ (_, _, day)) = self.selected_date.get() {
            let (year, month, _) = date;
             // Use .single() and if let Some(...)
             if let Some(dt) = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).single() {
                 if dt.weekday() == chrono::Weekday::Fri {
                    self.working_fridays.update(|fridays| {
                        if fridays.contains(&date) {
                            fridays.remove(&date);
                        } else {
                            // Check against the alternating pattern before inserting
                             if self.is_date_an_alternating_working_friday(date) {
                                 fridays.insert(date);
                            }
                        }
                    });
                 }
             }
        }
    }

    /// Check if a specific date is marked as a working Friday.
     pub fn is_friday_marked_as_working(self, date: CalendarDate) -> Signal<bool> { // Changed to take self by value (Copy)
         // Use Memo::new() and capture self (which is Copy)
         Memo::new(move |_| self.working_fridays.get().contains(&date)).into()
     }

    /// Determines if a given date falls on an "alternating working Friday".
    /// This assumes a pattern where Fridays alternate starting from the first Friday.
    pub fn is_date_an_alternating_working_friday(self, date: CalendarDate) -> bool { // Changed to take self by value (Copy)
        let (year, month, day) = date;
        // Use .single() and if let Some(...)
        if let Some(dt) = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).single() {
            if dt.weekday() == chrono::Weekday::Fri {
                if let Some(first_friday_day) = self.first_friday_date_in_month.get() {
                     // Check if day is >= first_friday_day before calculating difference
                    if day >= first_friday_day {
                        let week_diff = (day - first_friday_day) / 7;
                        // 0th diff (first Fri), 2nd diff (third Fri), etc. are working
                        return week_diff % 2 == 0;
                    }
                }
            }
        }
        false // Not a Friday or calculation failed
    }

    /// Calculate total working hours for the currently viewed month based on rules.
    /// Assumes 8 hours for Mon-Thu, 8 hours for alternating Fridays.
     pub fn calculate_monthly_working_hours(self) -> Signal<f32> { // Changed to take self by value (Copy)
        // Use Memo::new() and capture self (which is Copy)
        Memo::new(move |_| {
            let (year, month) = self.current_view_month.get();
            let days_in_month = get_days_in_month(year, month);
            let mut total_hours = 0.0;

            for day in 1..=days_in_month {
                let date = (year, month, day);
                // Use .single() and if let Some(...)
                if let Some(dt) = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).single() {
                    match dt.weekday() {
                        chrono::Weekday::Mon
                        | chrono::Weekday::Tue
                        | chrono::Weekday::Wed
                        | chrono::Weekday::Thu => {
                            total_hours += 8.0;
                        }
                        chrono::Weekday::Fri => {
                             if self.is_date_an_alternating_working_friday(date) {
                                total_hours += 8.0;
                            }
                        }
                        _ => {} // Weekends
                    }
                }
            }
            total_hours
        }).into()
     }
}

// Helper to provide and use the context (no changes)
pub fn provide_calendar_context(year: i32, month: u32) {
    provide_context(CalendarState::new(year, month));
}

pub fn use_calendar_context() -> Option<CalendarState> {
    use_context::<CalendarState>()
}
