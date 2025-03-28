use crate::components::day_cell::DayCell;
use crate::components::time_input_modal::TimeInputModal;
use crate::models::calendar_state::{use_calendar_context, CalendarDate};
use chrono::prelude::*;
use leptos::prelude::*;


// Can be moved to a shared utils module if used elsewhere
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

fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "January", 2 => "February", 3 => "March", 4 => "April", 5 => "May", 6 => "June",
        7 => "July", 8 => "August", 9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Invalid Month",
    }
}

// Use .single() to return Option
fn create_datetime(year: i32, month: u32, day: u32) -> Option<DateTime<Local>> {
    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).single()
}



// --- Calendar Component ---

#[component]
pub fn Calendar() -> impl IntoView {
    // Get state from context
    let state = use_calendar_context()
        .expect("CalendarState must be provided in context");

    let today = Local::now();
    let today_date: CalendarDate = (today.year(), today.month(), today.day());

    // Use signals derived from the state for rendering
    let current_view_month = state.current_view_month();
    let monthly_working_hours = state.calculate_monthly_working_hours();


    // Derived signal for calendar grid data (year, month, days_in_month, offset)
    let calendar_grid_data = Memo::new(move |_| {
        let (current_year, current_month) = current_view_month.get();
        let first_day_dt = create_datetime(current_year, current_month, 1)
            .unwrap_or_else(|| Local::now()); // Fallback to now if date fails
        let days_in_month = get_days_in_month(current_year, current_month);
        let first_weekday_offset = first_day_dt.weekday().num_days_from_monday(); // 0 = Mon, 6 = Sun

        (
            current_year,
            current_month,
            days_in_month,
            first_weekday_offset,
        )
    });

    // Signal for the range of days to render in the <For/> component
    let days_to_render = Memo::new(move |_| {
        let (_, _, days_in_month, _) = calendar_grid_data.get();
        1..=days_in_month
    });


    // Handler for day clicks - Now a simple closure
    let handle_day_click = move |date: CalendarDate| {
        state.select_date_and_show_modal(date);
    };


    view! {
        <div class="calendar">
             // Header section reacting to current_view_month changes
             { move || {
                  let (year, month) = current_view_month.get();
                  view! {
                     <h2>{ format!("{} {}", get_month_name(month), year) }</h2>
                        <div class="working-hours">
                            { format!("Target working hours this month: {:.1}", monthly_working_hours.get())}
                        </div>
                     // TODO: Add buttons here to change month/year
                     // <button on:click=move |_| state.set_view_month(year, month - 1)>"Prev"</button>
                     // <button on:click=move |_| state.set_view_month(year, month + 1)>"Next"</button>
                  }
             }}

            <div class="calendar-grid">
                // Static Weekday Headers
                <div class="weekday">"Mon"</div>
                <div class="weekday">"Tue"</div>
                <div class="weekday">"Wed"</div>
                <div class="weekday">"Thu"</div>
                <div class="weekday">"Fri"</div>
                <div class="weekday">"Sat"</div>
                <div class="weekday">"Sun"</div>

                // Render empty cells before the first day
                 { move || {
                      let (_, _, _, first_weekday_offset) = calendar_grid_data.get();
                      (0..first_weekday_offset)
                          .map(|_| view! { <div class="day empty"></div> })
                          .collect::<Vec<_>>()
                 }}

                 // Use <For/> component to render the DayCell component
                 <For
                    each=days_to_render
                    key=|day| *day
                    // Children closure now calculates props for DayCell
                    children=move |day: u32| {
                        // Get year and month reactively inside the children closure
                        let (year, month, _, _) = calendar_grid_data.get();
                        let date = (year, month, day);

                        // Calculate props here instead of inside DayCell
                        // Note: Accessing signals here might still contribute to complexity,
                        // but perhaps less than passing the whole state down.
                        let day_data = state.get_day_data_for_date(date).get(); // Get Option<DayData> directly
                        let is_expected_working_friday = state.is_date_an_alternating_working_friday(date);

                        view! {
                             <DayCell
                                date=date
                                today_date=today_date
                                // Pass calculated props
                                day_data=day_data
                                is_expected_working_friday=is_expected_working_friday
                                // Pass closure
                                on_click=handle_day_click.clone()
                            />
                        }
                    }
                />
            </div>

            // Render the modal component conditionally via its internal logic
             <TimeInputModal />
        </div>
    }
}

