use i18nrs::yew::use_translation;
use shared::{
    models::storage_entry::StorageEntry, style::default_colors::DefaultColors,
    utils::normalize::NormalizeDelay,
};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{
    function_component, html, use_effect_with, use_state, Callback, Html, MouseEvent, Properties,
};

use crate::{
    components::atoms::{
        button::Button, color_picker::ColorPicker, notification_input::NotificationInput,
        select::Select, setting::Setting, text_input::TextInput,
    },
    errors::setting_error::{SettingError, SettingErrorReason},
    models::setting_value::SettingValue,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<Result<SettingValue, SettingError>>,
    pub delay: StorageEntry,
    pub password_enabled: bool,
}

#[function_component(Settings)]
pub fn create_setting(prop: &Props) -> Html {
    let (i18n, set_language) = use_translation();

    let valid_languages = vec!["it".to_string(), "en".to_string(), "fr".to_string()];

    let show = use_state(|| false);
    let password_enabled = use_state(|| prop.password_enabled);

    let password_enabled_clone = password_enabled.clone();
    use_effect_with(prop.password_enabled, move |enabled| {
        password_enabled_clone.set(*enabled);
    });

    let listener = {
        let show_clone = show.clone();
        Callback::from(move |_| {
            show_clone.set(!*show_clone);
        })
    };

    let get_input_values = {
        let on_change = prop.callback.clone();
        Callback::from(move |input: SettingValue| {
            on_change.emit(Ok(input));
        })
    };

    let notification_delay_handle = {
        let on_change = prop.callback.clone();
        Callback::from(move |input: String| {
            if input.is_empty() {
                on_change.emit(Ok(SettingValue::new(
                    "delay".to_string(),
                    String::from("0"),
                    false,
                    String::new(),
                )));
            } else {
                if input.chars().any(|char| !matches!(char, '0'..='9' | '/')) {
                    on_change.emit(Err(SettingError {
                        field: "delay".to_string(),
                        error: SettingErrorReason::Format("dd/mm o dd".to_string()),
                    }))
                } else {
                    on_change.emit(Ok(SettingValue::new(
                        "delay".to_string(),
                        NormalizeDelay::convert_to_num(input),
                        false,
                        String::new(),
                    )));
                }
            }
        })
    };

    let password_handle = {
        let on_change = prop.callback.clone();
        Callback::from(move |input: String| {
            on_change.emit(Ok(SettingValue::new(
                "password".to_string(),
                input,
                false,
                String::new(),
            )));
        })
    };

    let password_enabled_handle = {
        let on_change = prop.callback.clone();
        let password_enabled_clone = password_enabled.clone();
        Callback::from(move |event: MouseEvent| {
            let input = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .checked();
            password_enabled_clone.set(!*password_enabled_clone);
            on_change.emit(Ok(SettingValue::new(
                "password-enabled".to_string(),
                input.to_string(),
                false,
                String::new(),
            )));
        })
    };

    let language_handle = {
        let valid_languages = valid_languages.clone();
        let on_change = prop.callback.clone();
        Callback::from(move |value: String| {
            if valid_languages.contains(&value) {
                set_language.emit(value.clone());
                on_change.emit(Ok(SettingValue::new(
                    "language".to_string(),
                    value,
                    false,
                    String::new(),
                )))
            } else {
                on_change.emit(Err(SettingError {
                    field: "language".to_string(),
                    error: SettingErrorReason::NonExistent,
                }));
            }
        })
    };

    html! {
        <div id="settings-container">
            <div id="settings-panel" class={
                if *show {
                    "show-panel"
                } else {
                    "hide-panel"
                }
            }>
                <Setting label={i18n.t("background")}><ColorPicker item="background-color" call_back={get_input_values.clone()} index=0 /></Setting>
                <Setting label={i18n.t("header")}><ColorPicker item="head-background-color" call_back={get_input_values.clone()} index=1 /></Setting>
                <Setting label={i18n.t("input")}><ColorPicker item="input-background-color" call_back={get_input_values.clone()} index=2 /></Setting>
                <Setting label={i18n.t("text")}><ColorPicker item="text-color" call_back={get_input_values.clone()} index=3 /></Setting>
                <Setting label={i18n.t("delay")}><NotificationInput name="days/minutes" on_change={notification_delay_handle.clone()} value = {
                    if prop.delay.value.contains("/") {
                        prop.delay.value.clone()
                    } else {
                        let num = NormalizeDelay::normalize(prop.delay.value.clone());
                        let days = num / 86400;
                        let minutes = num / 60 - days * 24 * 60;
                        format!("{}/{}", days, minutes)
                    }
                } /></Setting>
                <Setting label={i18n.t("enabled")}><input type="checkbox" onclick={password_enabled_handle} checked={*password_enabled} /></Setting>
                if *password_enabled {
                    <Setting label={"password"}><TextInput name="password" on_change={password_handle} color={DefaultColors::INPUT_BACKGROUND_COLOR.to_string()} /></Setting>
                }
                <Setting label={i18n.t("language")}>
                    <Select selections={valid_languages} onchange={language_handle} selected={i18n.get_current_language().to_string()}/>
                </Setting>
            </div>
            <Button onclick={listener.clone()} id="settings"><span class={format!("material-symbols-outlined {}", if *show {
                "spin"
            } else {
                ""
            })}>{"settings"}</span></Button>
        </div>
    }
}
