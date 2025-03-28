use leptos::prelude::*;
use time::Date;
use crate::utils::calendar::Calendar;

#[component]
pub fn Calendar() -> impl IntoView {
    let weekdays_names = Calendar::WEEKDAYS_NAMES
                    .into_iter()
                    .map(|d| {
                        view! { <div class="weekday">{d}</div> }
                    })
                    .collect::<Vec<_>>();
    let previous_month_days = (24..=28)
                    .into_iter()
                    .map(|day| {
                        view! { <div class="day previous-month">{day.to_string()}</div> }
                    })
                    .collect::<Vec<_>>();
    let today = Calendar::today();
    let current_month_days = (1..=31)
                    .into_iter()
                    .map(|day| {
                        let mut class_list = String::from("day");
                        let current_cell_date = Date::from_calendar_date(today.year(), today.month(), day);
                        if let Ok(cell_date) = current_cell_date {
                            if cell_date == today {
                                class_list.push_str(" today");
                            }
                        }
                        if Calendar::is_weekend(today.year(), today.month(), day) {
                            class_list.push_str(" weekend");
                        }
                        view! { <div class="day">{day.to_string()}</div> }
                    })
                    .collect::<Vec<_>>();

    view! {
        <div class="calendar">
            <h2>"Calendar"</h2>

            <div class="calendar-grid">
                {weekdays_names} {previous_month_days} {current_month_days}
            </div>
        </div>
    }
}
