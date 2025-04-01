use crate::utils::calendar::Calendar;
use leptos::prelude::*;

#[component]
pub fn Calendar() -> impl IntoView {
    view! {
        <div class="calendar">
            <div class="calendar-header">
                <span class="month-name">{Calendar::today().month().to_string()}</span>
                <button class="arrow">" < "</button>
                <button class="today-button">"Today"</button>
                <button class="arrow">" > "</button>
            </div>

            <div class="calendar-grid">
                {weekdays_names()} {previous_month_visible_days()} {current_month_days}
                {next_month_visible_days()}
            </div>
        </div>
    }
}

fn weekdays_names() -> Vec<impl IntoView> {
    Calendar::weekdays_names()
        .into_iter()
        .map(|d| {
            view! { <div class="weekday">{d}</div> }
        })
        .collect::<Vec<_>>()
}

fn previous_month_visible_days() -> Vec<impl IntoView> {
    let current = Calendar::today();
    let last_month = Calendar::from(current.year(), current.month().previous(), 1);
    let last_visible = last_month.last_visible_days(&current);
    (last_visible.0..=last_visible.1)
        .into_iter()
        .map(|day| {
            view! { <div class="day previous-month">{day.to_string()}</div> }
        })
        .collect::<Vec<_>>()
}

fn current_month_days() -> Vec<impl IntoView> {
    let current = Calendar::today();
    (1..=current.month().length(current.year()))
        .into_iter()
        .map(|day| {
            let mut class_list = String::from("day");
            if day == current.date.day() {
                class_list.push_str(" today");
            }
            if Calendar::is_weekend(current.year(), current.month(), day) {
                class_list.push_str(" weekend");
            }
            view! { <div class=class_list>{day.to_string()}</div> }
        })
        .collect::<Vec<_>>()
}

fn next_month_visible_days() -> Vec<impl IntoView> {
    let current = Calendar::today();
    let next_month = Calendar::from(current.year(), current.month().next(), 1);
    (1..=next_month.date.weekday().number_days_from_monday() + 1)
        .into_iter()
        .map(|day| {
            let mut class_list = String::from("day previous-month");
            if Calendar::is_weekend(current.year(), current.month(), day) {
                class_list.push_str(" weekend");
            }
            view! { <div class=class_list>{day.to_string()}</div> }
        })
        .collect::<Vec<_>>()
}
