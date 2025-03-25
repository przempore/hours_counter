use super::day_data::DayData;
use chrono::prelude::*;
use leptos::*;
use std::collections::{HashMap, HashSet};

/// Represents a calendar date as (year, month, day)
pub type CalendarDate = (i32, u32, u32);

#[derive(Clone)]
pub struct CalendarState {
    hours_data: RwSignal<HashMap<CalendarDate, DayData>>,
    working_fridays: RwSignal<HashSet<CalendarDate>>,
    selected_date: RwSignal<Option<CalendarDate>>,
    show_modal: RwSignal<bool>,
}

impl CalendarState {
    /// Initialize calendar state
    pub fn new() -> Self {
        Self {
            hours_data: create_rw_signal(HashMap::new()),
            working_fridays: create_rw_signal(HashSet::new()),
            selected_date: create_rw_signal(None),
            show_modal: create_rw_signal(false),
        }
    }

    /// Save time for a specific date
    pub fn save_time(&self, date: CalendarDate, hours: u32, minutes: u32) {
        self.hours_data.update(|data| {
            data.insert(date, DayData::new(hours, minutes));
        });
        self.show_modal.set(false);
    }

    /// Toggle working Friday status
    pub fn toggle_working_friday(&self, date: CalendarDate) {
        self.working_fridays.update(|fridays| {
            if fridays.contains(&date) {
                fridays.remove(&date);
            } else {
                fridays.insert(date);
            }
        });
    }

    pub fn get_hours_data(&self) -> RwSignal<HashMap<CalendarDate, DayData>> {
        self.hours_data
    }

    pub fn get_working_fridays(&self) -> RwSignal<HashSet<CalendarDate>> {
        self.working_fridays
    }

    pub fn get_selected_date(&self) -> RwSignal<Option<CalendarDate>> {
        self.selected_date
    }

    pub fn get_show_modal(&self) -> RwSignal<bool> {
        self.show_modal
    }
}

