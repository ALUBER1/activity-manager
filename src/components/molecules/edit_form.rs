use crate::{
    components::atoms::{button::Button, edit_input::EditInput, submit_button::SubmitButton},
    errors::form_error::{FormError, FormErrorReason},
};
use chrono::{Local, NaiveDate, NaiveTime};
use gloo::timers::callback::Timeout;
use i18nrs::yew::use_translation;
use shared::{models::record::Record, style::default_colors::DefaultColors};
use std::ops::Deref;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Result<Record, Vec<FormError>>>,
    pub cancel: Callback<bool>,
    pub record: Record,
}

#[function_component(EditForm)]
pub fn form(props: &Props) -> Html {
    let timer = 3000;
    let (i18n, _set_language) = use_translation();

    let value_state = use_state(|| props.record.clone());
    let name_color = use_state(|| String::new());
    let date_color = use_state(|| String::new());
    let time_color = use_state(|| String::new());

    let on_changename = {
        let cloned_value = value_state.clone();
        Callback::from(move |name| {
            cloned_value.set(Record {
                name,
                ..cloned_value.deref().clone()
            });
        })
    };

    let on_changedate = {
        let cloned_value = value_state.clone();
        Callback::from(move |date| {
            cloned_value.set(Record {
                date,
                ..cloned_value.deref().clone()
            });
        })
    };

    let on_changetime = {
        let cloned_value = value_state.clone();
        Callback::from(move |time| {
            cloned_value.set(Record {
                time,
                ..cloned_value.deref().clone()
            });
        })
    };

    let on_submit = {
        let cloned_value = value_state.clone();
        let clone_submit = props.on_submit.clone();
        let name_clone = name_color.clone();
        let date_clone = date_color.clone();
        let time_clone = time_color.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let data = cloned_value.deref().clone();
            let mut error_vec: Vec<FormError> = vec![];

            if NaiveDate::parse_from_str(&data.date, "%d/%m/%Y").is_err()
                || is_future_date(&data.date) == 0
            {
                date_clone.set(DefaultColors::INVALID_COLOR.to_string());
                let date_clone = date_clone.clone();
                Timeout::new(timer, move || {
                    date_clone.set("".to_string());
                })
                .forget();
                error_vec.push(FormError {
                    field: "date".to_string(),
                    error: if data.date.is_empty() {
                        FormErrorReason::Empty
                    } else if NaiveDate::parse_from_str(&data.date, "%d/%m/%Y").is_err() {
                        FormErrorReason::Format("dd/mm/aaaa".to_string())
                    } else if is_future_date(&data.date) == 0 {
                        FormErrorReason::Past
                    } else {
                        FormErrorReason::Fallback(data.date.clone())
                    },
                });
            }
            if data.name.is_empty() {
                name_clone.set(DefaultColors::INVALID_COLOR.to_string());
                let name_clone = name_clone.clone();
                Timeout::new(timer, move || {
                    name_clone.set("".to_string());
                })
                .forget();
                error_vec.push(FormError {
                    field: "name".to_string(),
                    error: FormErrorReason::Empty,
                });
            }
            if NaiveTime::parse_from_str(&data.time, "%H:%M").is_err()
                || !is_future_time(&data.time, is_future_date(&data.date))
            {
                time_clone.set(DefaultColors::INVALID_COLOR.to_string());
                let time_clone = time_clone.clone();
                Timeout::new(timer, move || {
                    time_clone.set("".to_string());
                })
                .forget();
                error_vec.push(FormError {
                    field: "time".to_string(),
                    error: if data.time.is_empty() {
                        FormErrorReason::Empty
                    } else if NaiveTime::parse_from_str(&data.time, "%H:%M").is_err() {
                        FormErrorReason::Format("hh:mm".to_string())
                    } else if !is_future_time(&data.time, is_future_date(&data.date)) {
                        FormErrorReason::Past
                    } else {
                        FormErrorReason::Fallback(data.time.clone())
                    },
                });
            }

            if error_vec.len() == 0 {
                clone_submit.emit(Ok(data));
            } else {
                clone_submit.emit(Err(error_vec))
            }
        })
    };

    html! {
        <form onsubmit={on_submit} class="edit-form">
            <div class="form-fields">
                <EditInput name={i18n.t("name")} on_change={on_changename} color={(*name_color).clone()} value={(*value_state).name.clone()} />
                <EditInput name={i18n.t("date2")} on_change={on_changedate} color={(*date_color).clone()} value={(*value_state).date.clone()} />
                <EditInput name={i18n.t("time")} on_change={on_changetime} color={(*time_color).clone()} value={(*value_state).time.clone()} />
            </div>
            <div class="editing-button-container">
                <SubmitButton id="submit"><span class="material-symbols-outlined">{"send"}</span></SubmitButton>
                <Button onclick={props.cancel.clone()} id="cancel"><span class="material-symbols-outlined">{"cancel"}</span></Button>
            </div>
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

fn is_future_time(time: &str, future: i8) -> bool {
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
