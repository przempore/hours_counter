`src/pages/calendar.rs`
```rust
// Add these imports at the top of src/pages/calendar.rs
#[cfg(feature = "ssr")]
use time::{Date, Month, OffsetDateTime, Weekday, Duration}; // Use time only on ssr
use leptos::prelude::*;
use leptos::server_fn::error::ServerFnError;

// This function runs ONLY on the server
#[server(GetCalendarData, "/api")]
pub async fn get_calendar_data(year: Option<i32>, month: Option<u8>) -> Result<CalendarData, ServerFnError> {
    // --- This block uses the `time` crate - ONLY RUNS ON SERVER ---
    #[cfg(feature = "ssr")]
    {
        use crate::utils::calendar::Calendar as TimeCalendar; // Use your original Calendar util here

        // Determine the target date
        let today = OffsetDateTime::now_local().map_err(|e| ServerFnError::ServerError(e.to_string()))?.date();
        let target_year = year.unwrap_or_else(|| today.year());
        // Ensure month is valid (1-12), default to today's month
        let target_month_u8 = month.unwrap_or_else(|| today.month().into());
        let target_month = Month::try_from(target_month_u8).map_err(|e| ServerFnError::ServerError(e.to_string()))?;

        // Use day 1 to represent the target month
        let first_day_of_month = Date::from_calendar_date(target_year, target_month, 1)
            .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

        let month_name = target_month.to_string();
        let year_num = target_year;

        let weekdays = TimeCalendar::weekdays_names(); // Get weekday names

        // --- Calculate days to display (Previous, Current, Next month) ---
        let mut days: Vec<DayData> = Vec::new();

        let days_in_month = target_month.length(target_year);
        let first_weekday = first_day_of_month.weekday();
        // Days from previous month
        let days_from_prev = first_weekday.number_days_from_monday();
        if days_from_prev > 0 {
            let prev_month_date = first_day_of_month.previous_month().ok_or_else(|| ServerFnError::ServerError("Could not get previous month".to_string()))?;
            let days_in_prev_month = prev_month_date.month().length(prev_month_date.year());
            for d in (days_in_prev_month - days_from_prev + 1)..=days_in_prev_month {
                 let date = Date::from_calendar_date(prev_month_date.year(), prev_month_date.month(), d).ok();
                 let is_weekend = date.map_or(false, |dt| matches!(dt.weekday(), Weekday::Saturday | Weekday::Sunday));
                 days.push(DayData { day: d, is_current_month: false, is_today: false, is_weekend });
            }
        }

        // Days from current month
        for d in 1..=days_in_month {
             let date = Date::from_calendar_date(year_num, target_month, d).ok();
             let is_today = date.map_or(false, |dt| dt == today);
             let is_weekend = date.map_or(false, |dt| matches!(dt.weekday(), Weekday::Saturday | Weekday::Sunday));
             days.push(DayData { day: d, is_current_month: true, is_today, is_weekend });
        }

        // Days from next month (ensure grid has 6 weeks = 42 days)
        let days_from_next = 42 - days.len();
        if days_from_next > 0 {
            let next_month_date = first_day_of_month.next_month().ok_or_else(|| ServerFnError::ServerError("Could not get next month".to_string()))?;
             for d in 1..=days_from_next {
                  let date = Date::from_calendar_date(next_month_date.year(), next_month_date.month(), d as u8).ok();
                  let is_weekend = date.map_or(false, |dt| matches!(dt.weekday(), Weekday::Saturday | Weekday::Sunday));
                  days.push(DayData { day: d as u8, is_current_month: false, is_today: false, is_weekend });
             }
        }
        // --- End of calculation block ---

        Ok(CalendarData {
            year: year_num,
            month: target_month_u8,
            month_name,
            days,
            weekdays,
        })
    }
    // This needs to be defined for non-ssr builds, though it won't be called there
    // if you only call this server fn from client-side actions/resources.
     #[cfg(not(feature = "ssr"))]
     {
        // Provide a dummy implementation or error for non-ssr builds if necessary
        // This arm shouldn't be hit if called correctly from client-side triggers
        Err(ServerFnError::ServerError("Server function called on client".to_string()))
     }

}
```


Update src/pages/calendar.rs to use the server function and the new data structure.

```rust
// In src/pages/calendar.rs

// Keep the imports for Serialize, Deserialize, server functions etc.
// Remove imports for time::Month etc. unless used *only* within the server fn block

// ... (Keep CalendarData, DayData structs) ...
// ... (Keep get_calendar_data server function) ...

#[component]
pub fn Calendar() -> impl IntoView {
    // Action to load calendar data, takes (Option<i32>, Option<u8>) as input
    let load_calendar_action = create_action(|input: &(Option<i32>, Option<u8>)| {
        let (year, month) = *input;
        // Call the server function. Leptos handles the network request.
        get_calendar_data(year, month)
    });

    // Resource to hold the *result* of the action
    // We map the action's result (Result<CalendarData, ServerFnError>) to Option<CalendarData>
    let calendar_data_resource: Resource<(Option<i32>, Option<u8>), Option<CalendarData>> = create_resource(
        // Source signal: react to changes in the action's input signal
        move || load_calendar_action.input().get(),
        // Fetcher: Call the action when the input changes
        move |input_tuple| async move {
            // If input is None (initial load), fetch today's data
            let (year, month) = input_tuple.unwrap_or((None, None));
             match get_calendar_data(year, month).await {
                Ok(data) => Some(data),
                Err(e) => {
                    // Log the error on the client side
                    leptos::logging::error!("Failed to load calendar data: {:?}", e);
                    None
                }
            }
        }
    );

    // Trigger initial load for the current month
    // Note: We need an effect because resources only run when their source signal changes
    // or during initial server rendering. Calling dispatch here triggers the initial client fetch.
    create_effect(move |_| {
       if load_calendar_action.input().get().is_none() && load_calendar_action.version().get() == 0 {
           load_calendar_action.dispatch((None, None));
       }
    });


    // --- Click Handlers ---
    // These now dispatch actions with the *next* desired year/month
    let go_to_previous_month = move |_| {
        if let Some(Ok(Some(current_data))) = calendar_data_resource.get() {
            let (prev_year, prev_month) = if current_data.month == 1 {
                (Some(current_data.year - 1), Some(12))
            } else {
                (Some(current_data.year), Some(current_data.month - 1))
            };
            load_calendar_action.dispatch((prev_year, prev_month));
        }
    };

    let go_to_next_month = move |_| {
         if let Some(Ok(Some(current_data))) = calendar_data_resource.get() {
            let (next_year, next_month) = if current_data.month == 12 {
                (Some(current_data.year + 1), Some(1))
            } else {
                (Some(current_data.year), Some(current_data.month + 1))
            };
            load_calendar_action.dispatch((next_year, next_month));
         }
    };

    let go_to_today = move |_| {
        load_calendar_action.dispatch((None, None)); // Request current month data
    };


    // --- Rendering ---
    view! {
        <div class="calendar">
            // Show a loading state while fetching
             <Suspense fallback=view!{<p>"Loading calendar..."</p>}>
                 // Use .get() on the resource to access its value
                {move || match calendar_data_resource.get() {
                    None => view! { <p>"Initializing..."</p> }.into_view(), // Still initializing or hasn't run yet
                    Some(Ok(Some(data))) => view! { // Data loaded successfully
                        <div class="calendar-header">
                             <span class="month-name">{format!("{} {}", data.month_name, data.year)}</span>
                             <button class="arrow" on:click=go_to_previous_month disabled=load_calendar_action.pending().get()> "<" </button>
                             <button class="today-button" on:click=go_to_today disabled=load_calendar_action.pending().get()>"Today"</button>
                             <button class="arrow" on:click=go_to_next_month disabled=load_calendar_action.pending().get()>" > "</button>
                         </div>
                         <div class="calendar-grid">
                            // Render weekdays from data
                            {data.weekdays.iter().map(|day_name| view! { <div class="weekday">{day_name}</div> }).collect::<Vec<_>>()}
                             // Render days from data
                            {data.days.iter().map(|day_info| {
                                let mut class_list = String::from("day");
                                if !day_info.is_current_month {
                                    class_list.push_str(" previous-month"); // Use a single class for non-current
                                }
                                if day_info.is_today {
                                    class_list.push_str(" today");
                                }
                                if day_info.is_weekend {
                                    class_list.push_str(" weekend");
                                }
                                view! { <div class=class_list>{day_info.day.to_string()}</div> }
                             }).collect::<Vec<_>>()}
                         </div>
                    }.into_view(),
                    Some(Ok(None)) => view! { <p>"No calendar data available."</p> }.into_view(), // Server returned None, shouldn't happen w/ current logic
                     Some(Err(e)) => view! { <p class="error">{format!("Error loading calendar: {:?}", e)}</p> }.into_view(), // Server returned an error
                 }}
             </Suspense>
         </div>
    }
}

// Remove the old rendering helper functions (weekdays_names, previous_month_visible_days, etc.)
// as the rendering logic is now directly using the Vec<DayData> from the resource.
```
