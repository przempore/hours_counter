// src/components/day_cell.rs
use crate::models::calendar_state::CalendarDate;
use crate::models::day_data::DayData; // Need DayData for the prop
use chrono::prelude::*;
use leptos::prelude::*;
use leptos::web_sys::MouseEvent;

#[component]
pub fn DayCell<F>(
    date: CalendarDate,
    today_date: CalendarDate,
    day_data: Option<DayData>, // Prop passed from parent
    is_expected_working_friday: bool, // Prop passed from parent
    on_click: F,
) -> impl IntoView
where
    F: Fn(CalendarDate) + Clone + 'static,
{
    let (year, month, day) = date;

    // Derive necessary data reactively based on props
    let dt = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).single();
    let weekday = dt.map(|d| d.weekday()); // Get Option<Weekday>

    let is_today = date == today_date;
    let is_weekend = weekday.map_or(false, |wd| matches!(wd, Weekday::Sat | Weekday::Sun));

    // Clone day_data *before* it's moved into the day_class closure
    let day_data_for_view = day_data.clone();

    // Class calculation now uses props directly.
    let day_class = Memo::new(move |_| {
        let mut classes = vec!["day"];
        // Use the original day_data (which will be moved into this closure)
        if day_data.is_some() { classes.push("has-hours"); }
        if is_today { classes.push("today"); }
        if is_weekend { classes.push("weekend"); }
        else if weekday == Some(Weekday::Fri) { // Compare Option<Weekday>
            if is_expected_working_friday { classes.push("working-friday"); }
            else { classes.push("non-working-friday"); }
        } else { // Also catches None case from weekday
            classes.push("workday");
        }
        classes.join(" ")
    });

    // Clone the on_click closure before the move closure for the event handler
    let on_click_clone = on_click.clone();

    view! {
         <div
            class=move || day_class.get()
            // Use the cloned closure and prefix unused `ev` with `_`
            on:click=move |_ev: MouseEvent| on_click_clone(date)
        >
            <span class="day-number">{day}</span>
             // Use the cloned day_data_for_view in this closure
            { move || day_data_for_view.clone().map(|data| view! {
                <span class="hours-label">
                    {format!("{:.1}h", data.to_f32_hours())}
                </span>
            })}
        </div>
    }
}
