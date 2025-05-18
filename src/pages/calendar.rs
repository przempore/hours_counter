// WiP on Gemini suggestion -> https://gemini.google.com/share/eb5573aeb17b

use leptos::logging::log;
use leptos::prelude::*;
use leptos::server_fn::error::ServerFnError;
#[cfg(feature = "ssr")]
use time::{Date, Duration, Month, OffsetDateTime, Weekday};

use crate::utils;
use crate::utils::types::{CalendarData, DayData};

#[server(GetCalendarData, "/api")]
pub async fn get_calendar_data(
    year: Option<i32>,
    month: Option<u8>,
) -> Result<utils::types::CalendarData, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::utils::calendar::Calendar as TimeCalendar;

        let today = OffsetDateTime::now_local()
            .expect("correct local time")
            .date();
        // let today = OffsetDateTime::now_local()
        //     .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        //     .date();
        log!("log from server: today: {}", today);
        let target_year = year.unwrap_or_else(|| today.year());
        let target_month_u8 = month.unwrap_or_else(|| today.month().into());
        let target_month = Month::try_from(target_month_u8)
            .expect("correct month number");

        let first_day_of_month = Date::from_calendar_date(target_year, target_month, 1)
            .expect("correct year or month");
            // .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

        let month_name = target_month.to_string();
        let year_num = target_year;

        let weekdays = TimeCalendar::weekdays_names();

        let days: Vec<DayData> = Vec::new();

        let days_in_month = target_month.length(target_year);
        let first_weekday = first_day_of_month.weekday();

        let days_from_previous_month = first_weekday.number_days_from_monday();
        if days_from_previous_month > 0 {}

        Ok(CalendarData {
            year: year_num,
            month: target_month_u8,
            month_name,
            days,
            weekdays,
        })
    }

    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::ServerError("Server function called on client".to_string()))
    }
}


#[component]
pub fn Calendar() -> impl IntoView {
    let load_calendar_data = Action::new(|input: &(Option<i32>, Option<u8>)| {
        let (year, month) = *input;
        get_calendar_data(year, month)
    });
    

    let calendar_data = Resource::new(
        move || load_calendar_data.input().get(),
        move |input| async move {
            let (year, month) = input.unwrap_or((None, None));
            match get_calendar_data(year, month).await {
                Ok(data) => Some(data),
                Err(e) => {
                    leptos::logging::error!("Error fetching calendar data: {:?}", e);
                    None
                }
            }
        } 
    );
    
    Effect::new(move |_| {
        if load_calendar_data.input().get().is_none() && load_calendar_data.version().get() == 0 {
            load_calendar_data.dispatch_local((None, None));
        }
    });

    // let go_to_previous_month = move |_| {
    //     if let Some(Some(current_data)) = calendar_data.get() {
    //         let (prev_year, prev_month) = if current_data.month == 1 {
    //             (current_data.year - 1, 12u8)
    //         } else {
    //             (current_data.year, current_data.month - 1)
    //         };
    //         load_calendar_data.dispatch_local((Some(prev_year), Some(prev_month)));
    //     }
    // };
    // 
    // let go_to_next_month = move |_| {
    //     if let Some(Some(current_data)) = calendar_data.get() {
    //         let (next_year, next_month) = if current_data.month == 12 {
    //             (current_data.year + 1, 1u8)
    //         } else {
    //             (current_data.year, current_data.month + 1)
    //         };
    //         load_calendar_data.dispatch_local((Some(next_year), Some(next_month)));
    //     }
    // };

    // let go_to_today = move |_| {
    //     load_calendar_data.dispatch_local((None, None));
    // };


    view! {
        <div class="calendar">
            // reading docs is more fruitfull than ai -> https://book.leptos.dev/async/11_suspense.html
            <Suspense fallback=move || view! { <p>"Loading calendar..."</p> }>
                <h2>"My Data"</h2>
                {move || Suspend::new(async move {
                    let (year, month) = load_calendar_data.input().get().unwrap();
                    log!("date in view: {:?} {:?}", year, month);
                })}
            </Suspense>
        </div>
    }
}

// use crate::utils::calendar::Calendar;
// use leptos::prelude::*;
// use time::{Month, OffsetDateTime};
// use leptos::logging::log;
//
// #[component]
// pub fn Calendar() -> impl IntoView {
//     let (current_date, set_current_date) = signal(Calendar::today());
//
//     let go_to_previous_month = move |_| {
//         log!("go_to_previous_month");
//         set_current_date.update(|date| {
//             let month = date.month().previous();
//             let year = if month == Month::December {
//                 date.year() - 1
//             } else {
//                 date.year()
//             };
//             *date = Calendar::from(year, month, 1);
//         });
//     };
//
//     let go_to_next_month = move |_| {
//         println!("go_to_next_month");
//         set_current_date.update(|date| {
//             let month = date.month().next();
//             let year = if month == Month::January {
//                 date.year() + 1
//             } else {
//                 date.year()
//             };
//             *date = Calendar::from(year, month, 1);
//         });
//     };
//
//     let go_to_today = move |_| set_current_date.set(Calendar::today());
//
//     view! {
//         <div class="calendar">
//             <div class="calendar-header">
//                 <span class="month-name">
//                     {format!("{} {}", current_date.get().month(), current_date.get().year())}
//                 </span>
//                 <button class="arrow" on:click=go_to_previous_month>
//                     " < "
//                 </button>
//                 <button class="today-button" on:click=go_to_today>
//                     "Today"
//                 </button>
//                 <button class="arrow" on:click=go_to_next_month>
//                     " > "
//                 </button>
//             </div>
//
//             <div class="calendar-grid">
//                 {weekdays_names()}
//                 {move || previous_month_visible_days(&current_date.get())}
//                 {move || current_month_days(&current_date.get())}
//                 {move || next_month_visible_days(&current_date.get())}
//             </div>
//         </div>
//     }
// }
//
// fn weekdays_names() -> Vec<impl IntoView> {
//     Calendar::weekdays_names()
//         .into_iter()
//         .map(|day| {
//             view! { <div class="weekday">{day}</div> }
//         })
//         .collect::<Vec<_>>()
// }
//
// fn previous_month_visible_days(current: &Calendar) -> Vec<impl IntoView> {
//     let last_month = Calendar::from(current.year(), current.month().previous(), 1);
//     let last_visible = last_month.last_visible_days(&current);
//     (last_visible.0..=last_visible.1)
//         .into_iter()
//         .map(|day| {
//             view! { <div class="day previous-month">{day.to_string()}</div> }
//         })
//         .collect::<Vec<_>>()
// }
//
// fn current_month_days(current: &Calendar) -> Vec<impl IntoView> {
//     (1..=current.month().length(current.year()))
//         .into_iter()
//         .map(|day| {
//             let mut class_list = String::from("day");
//             if day == current.date.day() {
//                 class_list.push_str(" today");
//             }
//             if Calendar::is_weekend(current.year(), current.month(), day) {
//                 class_list.push_str(" weekend");
//             }
//             view! { <div class=class_list>{day.to_string()}</div> }
//         })
//         .collect::<Vec<_>>()
// }
//
// fn next_month_visible_days(current: &Calendar) -> Option<Vec<impl IntoView>> {
//     let next_month = Calendar::from(current.year(), current.month().next(), 1);
//     let last_visible_days = 7 - next_month.date.weekday().number_days_from_monday();
//     if last_visible_days == 7u8 {
//         None
//     } else {
//         Some(
//             (1..=last_visible_days)
//                 .into_iter()
//                 .map(|day| {
//                     let mut class_list = String::from("day previous-month");
//                     if Calendar::is_weekend(current.year(), current.month(), day) {
//                         class_list.push_str(" weekend");
//                     }
//                     view! { <div class=class_list>{day.to_string()}</div> }
//                 })
//                 .collect::<Vec<_>>(),
//         )
//     }
// }
