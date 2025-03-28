use crate::pages::calendars::Calendar;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>"Hours Counter"</h1>
        <Calendar /> // Render the refactored Calendar component
    }
}
