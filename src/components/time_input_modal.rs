use crate::models::{calendar_state::CalendarState, calendar_state::CalendarDate};
use chrono::prelude::*;
use leptos::*;
use leptos::prelude::*;

#[component]
pub fn TimeInputModal(
    state: CalendarState,
    selected_date: CalendarDate,
    on_save: fn(u32, u32) -> (),
    on_close: fn() -> (),
) -> impl IntoView {
    let (year, month, day) = selected_date;
    let hours_data = state.get_hours_data();
    let working_fridays = state.get_working_fridays();

    let current_data = hours_data
        .get()
        .get(&selected_date)
        .cloned()
        .unwrap_or_default();

    let (input_hours, set_input_hours) = create_signal(current_data.hours);
    let (input_minutes, set_input_minutes) = create_signal(current_data.minutes);

    let current_date = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
    let is_friday = current_date.weekday() == Weekday::Fri;
    let is_working = working_fridays.get().contains(&selected_date);

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
                <div class="time-inputs">
                    <div class="hours-input">
                        <label>"Hours:"</label>
                        <input
                            type="number"
                            min="0"
                            max="23"
                            prop:value=input_hours
                            on:input=move |ev| {
                                if let Ok(hours) = event_target_value(&ev).parse::<u32>() {
                                    set_input_hours.set(hours);
                                }
                            }
                        />
                    </div>
                    <div class="minutes-input">
                        <label>"Minutes:"</label>
                        <input
                            type="number"
                            min="0"
                            max="59"
                            prop:value=input_minutes
                            on:input=move |ev| {
                                if let Ok(minutes) = event_target_value(&ev).parse::<u32>() {
                                    set_input_minutes.set(minutes);
                                }
                            }
                        />
                    </div>
                    <div
                        class="working-friday-toggle"
                        class:hidden=move || !is_friday
                    >
                        <label class="working-friday-label">
                            <input
                                type="checkbox"
                                prop:checked=is_working
                                on:change=move |_| {
                                    if is_friday {
                                        state.toggle_working_friday(selected_date);
                                    }
                                }
                            />
                            "Working Friday"
                        </label>
                    </div>
                </div>
                <div class="modal-buttons">
                    <button on:click=move |_| on_close()>"Cancel"</button>
                    <button on:click=move |_| {
                        on_save(input_hours.get(), input_minutes.get())
                    }>"Save"</button>
                </div>
            </div>
        </div>
    }
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

