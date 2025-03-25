use leptos::prelude::*;
use crate::pages::calendars::Calendar;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hours Counter"</h1>
        <Calendar/>
    }
}


