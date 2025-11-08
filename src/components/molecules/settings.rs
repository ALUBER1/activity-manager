use i18nrs::yew::use_translation;
use shared::{models::storage_entry::StorageEntry, style::default_colors::DefaultColors, utils::normalize::NormalizeDelay};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Callback, Html, MouseEvent, Properties, function_component, html, use_effect_with, use_state};

use crate::{components::atoms::{button::Button, color_picker::ColorPicker, notification_input::NotificationInput, setting::Setting, text_input::TextInput}, models::setting_value::SettingValue};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub callback: Callback<SettingValue>,
    pub delay: StorageEntry,
    pub password_abilitated: bool
}

#[function_component(Settings)]
pub fn create_setting(prop: &Props) -> Html {
    let (i18n, _set_language) = use_translation();

    let show = use_state(||false);
    let password_abilitated = use_state(||prop.password_abilitated);

    let password_abilitated_clone = password_abilitated.clone();
    use_effect_with(prop.password_abilitated, move |abilitated| {
        password_abilitated_clone.set(*abilitated);
    });
    
    let show_clone = show.clone();
    let listener = Callback::from(move |_| {
        show_clone.set(!*show_clone);
    });

    let on_change = prop.callback.clone();
    let get_input_values = Callback::from(move |input: SettingValue|{
        on_change.emit(input);
    });

    let on_change = prop.callback.clone();
    let notification_delay_handle = Callback::from(move |input: String|{
        on_change.emit(SettingValue::new("delay".to_string(), NormalizeDelay::convert_to_num(input), false, String::new()));
    });

    let on_change = prop.callback.clone();
    let password_handle = Callback::from(move |input: String|{
        on_change.emit(SettingValue::new("password".to_string(), input, false, String::new()));
    });

    let on_change = prop.callback.clone();
    let password_abilitated_clone = password_abilitated.clone();
    let password_abilitated_handle = Callback::from(move |event: MouseEvent|{
        let input = event.target().unwrap().unchecked_into::<HtmlInputElement>().checked();
        password_abilitated_clone.set(!*password_abilitated_clone);
        on_change.emit(SettingValue::new("password-abilitated".to_string(), input.to_string(), false, String::new()));
    });

    html!{
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
                <Setting label={i18n.t("abilitated")}><input type="checkbox" onclick={password_abilitated_handle} checked={*password_abilitated} /></Setting>
                if *password_abilitated {
                    <Setting label={"password"}><TextInput name="password" on_change={password_handle} color={DefaultColors::INPUT_BACKGROUND_COLOR.to_string()} value={""}/></Setting> 
                }
            </div>
            <Button onclick={listener.clone()} id="settings"><span class={format!("material-symbols-outlined {}", if *show {
                "spin"
            } else {
                ""
            })}>{"settings"}</span></Button>
        </div>
    }
}