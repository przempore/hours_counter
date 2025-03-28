use leptos::prelude::*;
use crate::models::day_data::DayData;
use crate::models::calendar_state::CalendarDate;

#[component]
pub fn TimeInputPanel(
    selected_date: CalendarDate,
    day_data: Option<DayData>,
    #[prop(into)] on_save: Callback<(u32, u32)>,
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    let (hours, set_hours) = create_signal(day_data.as_ref().map_or(0, |d| d.hours()));
    let (minutes, set_minutes) = create_signal(day_data.as_ref().map_or(0, |d| d.minutes()));

    let handle_save = move |_| {
        on_save.call((hours.get(), minutes.get()));
    };

    let handle_close = move |_| {
        on_close.call(());
    };

    view! {
        <div class="time-input-panel">
            <div class="panel-content">
                <h3>"Enter Working Hours"</h3>
                <div class="time-inputs">
                    <div>
                        <label>"Hours:"</label>
                        <input
                            type="number"
                            min="0"
                            max="24"
                            value=hours
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                    set_hours(value.min(24));
                                }
                            }
                        />
                    </div>
                    <div>
                        <label>"Minutes:"</label>
                        <input
                            type="number"
                            min="0"
                            max="59"
                            value=minutes
                            on:input=move |ev| {
                                if let Ok(value) = event_target_value(&ev).parse::<u32>() {
                                    set_minutes(value.min(59));
                                }
                            }
                        />
                    </div>
                </div>
                <div class="panel-buttons">
                    <button on:click=handle_close>"Cancel"</button>
                    <button on:click=handle_save>"Save"</button>
                </div>
            </div>
        </div>
    }
}

