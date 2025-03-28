use crate::models::calendar_state::CalendarState;
// Removed LocalResult import below
use chrono::{Datelike, Local, TimeZone, Weekday};
use leptos::prelude::*;

// Helper function (can be moved to a utils module)
fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "January", 2 => "February", 3 => "March", 4 => "April", 5 => "May", 6 => "June",
        7 => "July", 8 => "August", 9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Invalid Month",
    }
}


#[component]
pub fn TimeInputModal() -> impl IntoView {
    // Get state from context
    let state = use_context::<CalendarState>()
        .expect("CalendarState context should be provided");

    let selected_date = state.get_selected_date();

    // Create local signals for the input fields, updated when the selected date changes
    // Use signal()
    let (input_hours, set_input_hours) = signal(0_u32);
    let (input_minutes, set_input_minutes) = signal(0_u32);

     // Effect to update inputs when selected_date changes or modal becomes visible
     // Use Effect::new()
    Effect::new(move |_| {
        if let Some(date) = selected_date.get() {
            // Fetch existing data directly when the effect runs
             let existing_data = state.hours_map().get().get(&date).cloned().unwrap_or_default();
            set_input_hours.set(existing_data.hours());
            set_input_minutes.set(existing_data.minutes());
        } else {
             // Reset when no date is selected (modal likely closed)
             set_input_hours.set(0);
             set_input_minutes.set(0);
        }
    });


    let handle_save = move |_| {
        state.save_time_for_selected(input_hours.get(), input_minutes.get());
    };

    let handle_close = move |_| {
        state.close_modal();
    };

    let handle_working_friday_toggle = move |_| {
        state.toggle_working_friday_for_selected();
    };

    // Derived signals for display logic
    // Use Memo::new()
    let is_friday = Memo::new(move |_| {
         selected_date.get().map_or(false, |(y, m, d)| {
             // Use .single() and map_or on the resulting Option
             Local.with_ymd_and_hms(y, m, d, 0, 0, 0).single()
                  .map_or(false, |dt| dt.weekday() == Weekday::Fri)
         })
    });

    // Use the state's method to check if the date *should* be a working Friday
    // Use Memo::new()
    let is_alternating_working_friday = Memo::new(move |_| {
        selected_date.get().map_or(false, |date| state.is_date_an_alternating_working_friday(date))
    });

    // Check if it's *marked* as working in the state
    // Use Memo::new()
    let is_marked_as_working = Memo::new(move |_| {
         selected_date.get().map_or(false, |date| state.is_friday_marked_as_working(date).get())
    });


    // Combine conditions: Show toggle only if it's a Friday that *should* be a working day based on the pattern
    // Use Memo::new()
    let show_working_friday_toggle = Memo::new(move|_| is_friday.get() && is_alternating_working_friday.get());


    view! {
        // Use a conditional signal to show/hide the modal
        { move || selected_date.get().map(|(year, month, day)| {
            view! {
                <div class="modal-backdrop"> // Added backdrop for better styling
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
                                        max="23" // Standard hours max
                                        prop:value=move || input_hours.get() // Use getter for prop
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            if let Ok(h) = val.parse::<u32>() {
                                                set_input_hours.set(h);
                                            } else if val.is_empty() {
                                                 set_input_hours.set(0); // Handle empty input
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
                                        step="1" // Usually minutes are whole numbers
                                        prop:value=move || input_minutes.get() // Use getter for prop
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            if let Ok(m) = val.parse::<u32>() {
                                                set_input_minutes.set(m);
                                            } else if val.is_empty() {
                                                set_input_minutes.set(0); // Handle empty input
                                            }
                                        }
                                    />
                                </div>
                                <div
                                    class="working-friday-toggle"
                                    // Use hidden attribute for better semantics/accessibility
                                    hidden=move || !show_working_friday_toggle.get()
                                >
                                    <label class="working-friday-label">
                                        <input
                                            type="checkbox"
                                            // Checked if it *is* marked as working
                                            prop:checked=move || is_marked_as_working.get()
                                            on:change=handle_working_friday_toggle
                                        />
                                        "Working Friday"
                                    </label>
                                </div>
                            </div>
                            <div class="modal-buttons">
                                <button on:click=handle_close>"Cancel"</button>
                                <button on:click=handle_save>"Save"</button>
                            </div>
                        </div>
                    </div>
                </div>
            }
         })
        }
    }
}
