use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{components::molecules::{form::Form, password_screen::PasswordScreen, record_list::RecordList, settings::Settings, title_bar::TitleBar}, models::setting_value::SettingValue, utils::{functions::Functions, helper::*, logger::log}};
use shared::{models::{record::Record, storage_entry::StorageEntry}, utils::normalize::NormalizeDelay};

#[wasm_bindgen(module="/src/js/variable_modify.js")]
extern "C" {
    fn change_background(input: &str);
}

#[wasm_bindgen(module="/src/js/pickr.mjs")]
extern "C" {
    pub fn init_pickr(id: String, def: String);
}

#[function_component(App)]
pub fn app() -> Html {
    
    let record_list: UseStateHandle<Vec<Record>> = use_state(||Vec::new());
    let delay = use_state(||StorageEntry::new_delay("0/60".to_string()));
    let temp = use_state(||StorageEntry::default());
    let password_abilitated = use_state(||StorageEntry::default());
    let correct = use_state(||StorageEntry::new("password".to_string(), false.to_string()));
    
    let clone_list = record_list.clone();
    let delay_clone = delay.clone();
    let temp_clone = temp.clone();
    let password_abilitated_clone = password_abilitated.clone();
    
    use_effect_with((), move |_|{
        spawn_local(async move {
            invoke_function_async("create_database", None, None).await;
            invoke_function_async("initialize_database", None, None).await;  
            invoke_function_vec("get_all_records", Some(clone_list.clone()), None).await;
            invoke_function("notification_loop", None, None);

            invoke_function_store("get_storage", Some(delay_clone.clone()), Some(StorageEntry::new_delay(String::new()))).await;
            invoke_function_store("get_storage", Some(temp_clone.clone()), Some(StorageEntry::new("text-color".to_string(), String::new()))).await;
            invoke_function_store("get_storage", Some(temp_clone.clone()), Some(StorageEntry::new("background-color".to_string(), String::new()))).await;
            invoke_function_store("get_storage", Some(temp_clone.clone()), Some(StorageEntry::new("input-background-color".to_string(), String::new()))).await;
            invoke_function_store("get_storage", Some(temp_clone.clone()), Some(StorageEntry::new("head-background-color".to_string(), String::new()))).await;
            invoke_function_store("get_storage", Some(password_abilitated_clone.clone()), Some(StorageEntry::new("password-abilitated".to_string(), String::new()))).await;
        });
        
        ||{}
    });
    
    use_effect_with((*temp).clone(), move |temp| {
        if !(*temp).value.is_empty() {
            let tmp = SettingValue::from((*temp).clone()).serialize();
            change_background(&tmp);
        }
    });
    
    let clone_list = record_list.clone();
    let on_submit = Callback::from(move |record: Record| {
        let record = record.clone();
        let clone_list = clone_list.clone();
        spawn_local(async move{
            invoke_function_async("add_record", None, Some(Record { uuid: "".to_string(), ..record})).await;
            invoke_function_vec("get_all_records", Some(clone_list.clone()), None).await;
            ()
        });
    });

    let clone_list = record_list.clone();
    let delete_handler = Callback::from(move |record: Record|{
        let record = record.clone();
        let clone_list = clone_list.clone();
        spawn_local(async move {
            invoke_function_async("delete_record", None, Some(record)).await;
            invoke_function_vec("get_all_records", Some(clone_list.clone()), None).await;
            ()
        });
    });

    let title_handler = Callback::from(move |function: Functions| {
        match function {
            Functions::Close => invoke_function("close_app", None, None),
            Functions::Minimize => invoke_function("minimize_app", None, None),
            Functions::Maximize => invoke_function("maximize_app", None, None),
            Functions::Tray => invoke_function("tray_app", None, None),
        }
    });

    let clone_list = record_list.clone();
    let edit_handler = Callback::from(move |record: Record|{
        let record = record.clone();
        let clone_list = clone_list.clone();
        spawn_local(async move {
            invoke_function_async("delete_record", None, Some(record)).await;
            invoke_function_vec("get_all_records", Some(clone_list.clone()), None).await;
            ()
        });
    });

    
    let delay_clone = delay.clone();
    let settings_handler = Callback::from(move |input: SettingValue|{
        let delay_clone = delay_clone.clone();
        let input = input.clone();
        spawn_local(async move {
            if input.from_color_picker {
                change_background(&input.serialize());
                invoke_function_store("store_storage", None, Some(StorageEntry::new_color(input.value.clone(), input.setting))).await;
            } else {
                match &*(input.setting) {
                    "delay" => {
                        invoke_function_store("store_storage", None, Some(StorageEntry::new_delay(input.value.clone()))).await;
                        delay_clone.set(StorageEntry::new_delay(NormalizeDelay::convert_to_string(input.value)));
                    },
                    "password" => {
                        invoke_function_store("store_password", None, Some(StorageEntry::new(input.setting.clone(), input.value.clone()))).await;
                    },
                    "password-abilitated" => {
                        if input.value.eq("true") {
                            invoke_function_store("store_storage", None, Some(StorageEntry::new("password-abilitated".to_string(), "true".to_string()))).await;
                        } else {
                            invoke_function_store("store_storage", None, Some(StorageEntry::new("password-abilitated".to_string(), "".to_string()))).await;
                        }
                    },
                    _ => ()
                }
            }
        });
    });

    let correct_clone = correct.clone();
    let password_handler = Callback::from(move |password: String| {
        log("password: {}", &[&password]);
        let correct = correct_clone.clone();
        spawn_local(async move {
            invoke_function_store("verify", Some(correct.clone()), Some(StorageEntry::new("password".to_string(), password))).await;
        });
    });

    log("password abilitated: $0, correct: $1", &[&NormalizeDelay::normalize_color((*password_abilitated).clone().value), &(*correct).clone().value]);

    html! {
        <div id="main">
            <div id="fixed">
                <TitleBar on_click={title_handler}></TitleBar>
                <div id="form">
                    <Form on_submit = {on_submit}/>
                </div>
            </div>
            <div id="non-fixed">
                <div id="record-list">
                    <RecordList list = {(*record_list).clone()} delete_callback = {delete_handler} edit_callback = {edit_handler} />
                </div>
            </div>
            <Settings callback={settings_handler} delay={(*delay).clone()} />
            if NormalizeDelay::normalize_color((*password_abilitated).clone().value).eq("true") {
                <PasswordScreen invalid={
                    if (*correct).value.eq("false") {
                        true
                    } else {
                        false
                    }
                } callback={password_handler} />
            }
        </div>
    }
}
