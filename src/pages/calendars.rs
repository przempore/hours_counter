use chrono::prelude::*;
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct DayData {
    hours: f32,
}

fn get_days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => {
            if year % 4 == 0 {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn create_datetime(year: i32, month: u32, day: u32) -> DateTime<Local> {
    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!(),
    }
}

#[component]
pub fn Calendar() -> impl IntoView {
    let today: DateTime<Local> = Local::now();
    let current_month = today.month();
    let current_year = today.year();

    // State for storing hours data
    let (hours_data, set_hours_data) = signal(HashMap::<(i32, u32, u32), DayData>::new());

    // State for modal
    let (selected_date, set_selected_date) = signal::<Option<(i32, u32, u32)>>(None);
    let (show_modal, set_show_modal) = signal(false);

    // Handler for day clicks
    let handle_day_click = move |year: i32, month: u32, day: u32| {
        set_selected_date.set(Some((year, month, day)));
        set_show_modal.set(true);
    };

    // Handler for saving hours
    let save_hours = move |hours: f32| {
        if let Some((year, month, day)) = selected_date.get() {
            set_hours_data.update(|data| {
                data.insert((year, month, day), DayData { hours });
            });
        }
        set_show_modal.set(false);
    };

    // Get the first day of the month
    let first_day = create_datetime(current_year, current_month, 1);
    let days_in_month = get_days_in_month(current_year, current_month);

    // Get the weekday of the first day (0 = Monday, 6 = Sunday)
    let first_weekday = first_day.weekday().num_days_from_monday();

    let month_name = get_month_name(current_month);

    view! {
        <div class="calendar">
            <h2>{month_name} " " {current_year}</h2>
            <div class="working-hours">
                {"Working hours this month: "}
                {calculate_working_hours(current_year, current_month)}
            </div>
            <div class="calendar-grid">
                <div class="weekday">"Mon"</div>
                <div class="weekday">"Tue"</div>
                <div class="weekday">"Wed"</div>
                <div class="weekday">"Thu"</div>
                <div class="weekday">"Fri"</div>
                <div class="weekday">"Sat"</div>
                <div class="weekday">"Sun"</div>

                // Empty cells for days before the first of the month
                {(0..first_weekday)
                    .map(|_| view! { <div class="day empty"></div> })
                    .collect::<Vec<_>>()}

                // Days of the month
                {(1..=days_in_month)
                    .map(move |day| {
                        let is_today = day == today.day();
                        let day_data = hours_data
                            .get()
                            .get(&(current_year, current_month, day))
                            .cloned();
                        let current_date = create_datetime(current_year, current_month, day);
                        let weekday = current_date.weekday();
                        let is_weekend = matches!(weekday, Weekday::Sat | Weekday::Sun);
                        let is_working_friday = if weekday == Weekday::Fri {
                            let first_friday_date = (7
                                - (first_day.weekday().num_days_from_monday() as i32 - 4))
                                .rem_euclid(7) + 1;
                            let friday_number = (day - first_friday_date as u32) / 14 + 1;
                            friday_number % 2 != 0
                        } else {
                            false
                        };
                        let day_class = match (is_today, day_data.is_some(), is_weekend, weekday) {
                            (true, true, _, _) => "day today has-hours",
                            (true, false, _, _) => "day today",
                            (_, true, true, _) => "day has-hours weekend",
                            (_, false, true, _) => "day weekend",
                            (_, true, false, Weekday::Fri) if !is_working_friday => {
                                "day has-hours non-working-friday"
                            }
                            (_, false, false, Weekday::Fri) if !is_working_friday => {
                                "day non-working-friday"
                            }
                            (_, true, false, _) => "day has-hours workday",
                            (_, false, false, _) => "day workday",
                        };
                        let year = current_year;
                        let month = current_month;
                        view! {
                            // For Fridays, check if it's a working Friday

                            <div
                                class=day_class
                                on:click=move |_| handle_day_click(year, month, day)
                            >
                                <span class="day-number">{day}</span>
                                {day_data
                                    .map(|data| {
                                        view! {
                                            <span class="hours-label">
                                                {format!("{:.1}h", data.hours)}
                                            </span>
                                        }
                                    })}
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>

            // Hours Input Modal
            {move || {
                show_modal
                    .get()
                    .then(|| {
                        let (year, month, day) = selected_date.get().unwrap();
                        let current_hours = hours_data
                            .get()
                            .get(&(year, month, day))
                            .map(|data| data.hours)
                            .unwrap_or(0.0);
                        let (input_hours, set_input_hours) = signal(current_hours);
                        view! {
                            <div class="modal">
                                <div class="modal-content">
                                    <h3>
                                        {format!(
                                            "Enter hours for {} {}, {}",
                                            get_month_name(month),
                                            day,
                                            year,
                                        )}
                                    </h3>
                                    <input
                                        type="number"
                                        step="0.5"
                                        min="0"
                                        max="24"
                                        prop:value=input_hours
                                        on:input=move |ev| {
                                            if let Ok(hours) = event_target_value(&ev).parse::<f32>() {
                                                set_input_hours.set(hours);
                                            }
                                        }
                                    />
                                    <div class="modal-buttons">
                                        <button on:click=move |_| {
                                            set_show_modal.set(false)
                                        }>"Cancel"</button>
                                        <button on:click=move |_| save_hours(
                                            input_hours.get(),
                                        )>"Save"</button>
                                    </div>
                                </div>
                            </div>
                        }
                    })
            }}
        </div>
    }
}

fn calculate_working_hours(year: i32, month: u32) -> f32 {
    let mut total_hours = 0.0;
    let first_day = create_datetime(year, month, 1);
    let days_in_month = get_days_in_month(year, month);

    // Determine if first Friday of the month is a working Friday
    // We'll consider odd-numbered Fridays as working days
    let first_friday_date =
        (7 - (first_day.weekday().num_days_from_monday() as i32 - 4)).rem_euclid(7) + 1;

    for day in 1..=days_in_month {
        let current_day = create_datetime(year, month, day);
        let weekday = current_day.weekday();

        match weekday {
            Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu => {
                total_hours += 8.0;
            }
            Weekday::Fri => {
                // Check if it's a working Friday (every other Friday)
                let friday_number = (day - first_friday_date as u32) / 14 + 1;
                if friday_number % 2 != 0 {
                    total_hours += 8.0;
                }
            }
            _ => {} // Weekend days
        }
    }

    total_hours
}
