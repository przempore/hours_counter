use crate::utils::calendar::Calendar;
use leptos::prelude::*;
use time::Month;
use leptos::logging::log;

#[component]
pub fn Calendar() -> impl IntoView {
    let (current_date, set_current_date) = signal(Calendar::today());

    let go_to_previous_month = move |_| {
        log!("go_to_previous_month");
        set_current_date.update(|date| {
            let month = date.month().previous();
            let year = if month == Month::December {
                date.year() - 1
            } else {
                date.year()
            };
            *date = Calendar::from(year, month, 1);
        });
    };

    let go_to_next_month = move |_| {
        println!("go_to_next_month");
        set_current_date.update(|date| {
            let month = date.month().next();
            let year = if month == Month::January {
                date.year() + 1
            } else {
                date.year()
            };
            *date = Calendar::from(year, month, 1);
        });
    };

    let go_to_today = move |_| set_current_date.set(Calendar::today());

    view! {
        <div class="calendar">
            <div class="calendar-header">
                <span class="month-name">
                    {format!("{} {}", current_date.get().month(), current_date.get().year())}
                </span>
                <button class="arrow" on:click=go_to_previous_month>
                    " < "
                </button>
                <button class="today-button" on:click=go_to_today>
                    "Today"
                </button>
                <button class="arrow" on:click=go_to_next_month>
                    " > "
                </button>
            </div>

            <div class="calendar-grid">
                {weekdays_names()}
                {move || previous_month_visible_days(&current_date.get())}
                {move || current_month_days(&current_date.get())}
                {move || next_month_visible_days(&current_date.get())}
            </div>
        </div>
    }
}

fn weekdays_names() -> Vec<impl IntoView> {
    Calendar::weekdays_names()
        .into_iter()
        .map(|day| {
            view! { <div class="weekday">{day}</div> }
        })
        .collect::<Vec<_>>()
}

fn previous_month_visible_days(current: &Calendar) -> Vec<impl IntoView> {
    let last_month = Calendar::from(current.year(), current.month().previous(), 1);
    let last_visible = last_month.last_visible_days(&current);
    (last_visible.0..=last_visible.1)
        .into_iter()
        .map(|day| {
            view! { <div class="day previous-month">{day.to_string()}</div> }
        })
        .collect::<Vec<_>>()
}

fn current_month_days(current: &Calendar) -> Vec<impl IntoView> {
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

fn next_month_visible_days(current: &Calendar) -> Option<Vec<impl IntoView>> {
    let next_month = Calendar::from(current.year(), current.month().next(), 1);
    let last_visible_days = 7 - next_month.date.weekday().number_days_from_monday();
    if last_visible_days == 7u8 {
        None
    } else {
        Some(
            (1..=last_visible_days)
                .into_iter()
                .map(|day| {
                    let mut class_list = String::from("day previous-month");
                    if Calendar::is_weekend(current.year(), current.month(), day) {
                        class_list.push_str(" weekend");
                    }
                    view! { <div class=class_list>{day.to_string()}</div> }
                })
                .collect::<Vec<_>>(),
        )
    }
}
