use std::ops::Deref;
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use crate::components::{atoms::{submit_button::SubmitButton, text_input::TextInput, button::Button}};
use shared::{models::record::Record, style::default_colors::DefaultColors};
use chrono::{Local, NaiveDate, NaiveTime};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Record>,
    pub cancel: Callback<bool>,
    pub record: Record
}

#[function_component(EditForm)]
pub fn form(props: &Props) -> Html{
    let timer = 3000;

    let value_state = use_state(||props.record.clone());
    let name_color = use_state(||String::new());
    let date_color = use_state(||String::new());
    let time_color = use_state(||String::new());

    let cloned_value = value_state.clone();
    let on_changename = Callback::from(move |name|{
        cloned_value.set(Record { name, ..cloned_value.deref().clone()});
    });

    let cloned_value = value_state.clone();
    let on_changedate = Callback::from(move |date|{
        cloned_value.set(Record { date,..cloned_value.deref().clone()});
    });

    let cloned_value = value_state.clone();
    let on_changetime = Callback::from(move |time|{
        cloned_value.set(Record { time,..cloned_value.deref().clone()});
    });

    let cloned_value = value_state.clone();
    let clone_submit = props.on_submit.clone();
    let name_clone = name_color.clone();
    let date_clone = date_color.clone();
    let time_clone = time_color.clone();
    let on_submit = Callback::from(move |event: SubmitEvent|{
        event.prevent_default();
        let data = cloned_value.deref().clone();
        let mut correct = true;
        
        if NaiveDate::parse_from_str(&data.date, "%d/%m/%Y").is_err() || is_future_date(&data.date) == 0 {
            date_clone.set(DefaultColors::INVALID_COLOR.to_string());
            correct = false;
            let date_clone = date_clone.clone();
            Timeout::new(timer, move || {
                date_clone.set("".to_string());
            }).forget();
        }
        if data.name.is_empty() {
            name_clone.set(DefaultColors::INVALID_COLOR.to_string());
            correct = false;
            let name_clone = name_clone.clone();
            Timeout::new(timer, move || {
                name_clone.set("".to_string());
            }).forget();
        }
        if NaiveTime::parse_from_str(&data.time, "%H:%M").is_err() || !is_future_time(&data.time, is_future_date(&data.date)) {
            time_clone.set(DefaultColors::INVALID_COLOR.to_string());
            correct = false;
            let time_clone = time_clone.clone();
            Timeout::new(timer, move || {
                time_clone.set("".to_string());
            }).forget();
        }
        if correct {
            clone_submit.emit(data);
        }
    });

    html!{
        <form onsubmit={on_submit}>
            <TextInput name="name" on_change={on_changename} color={(*name_color).clone()} value={(*value_state).name.clone()} />
            <TextInput name="date (DD/MM/YYYY)" on_change={on_changedate} color={(*date_color).clone()} value={(*value_state).date.clone()} />
            <TextInput name="time (HH:MM)" on_change={on_changetime} color={(*time_color).clone()} value={(*value_state).time.clone()} />
            <SubmitButton id="submit"><span class="material-symbols-outlined">{"send"}</span></SubmitButton>
            <Button onclick={props.cancel.clone()} id="cancel"><span class="material-symbols-outlined">{"cancel"}</span></Button>
        </form>
    }
}

fn is_future_date(date: &str) -> i8 {
    if let Ok(parsed) = NaiveDate::parse_from_str(date, "%d/%m/%Y") {
        let local = Local::now().date_naive();
        if parsed > local {
            return 1;
        } else if parsed == local {
            return 2;
        }
    } 

    0
}

fn is_future_time(time: &str, future : i8) -> bool {
    if future == 1 {
        return true;
    } else if future == 2 {
        if let Ok(parsed) = NaiveTime::parse_from_str(time, "%H:%M") {  
            let local = Local::now().time();
            return parsed > local;
        }
    }
    false
}