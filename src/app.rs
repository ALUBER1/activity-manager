use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::{components::molecules::{form::Form, record_list::RecordList, settings::Settings, title_bar::TitleBar}, utils::{functions::Functions, helper::*}};
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
    
    let clone_list = record_list.clone();
    let delay_clone = delay.clone();
    use_effect_with((), move |_|{
        spawn_local(async move {
            invoke_function_async("create_database", None, None).await;
            invoke_function_async("initialize_database", None, None).await;  
            invoke_function_vec_async("get_all_records", Some(clone_list.clone()), None).await;
            invoke_function("notification_loop", None, None);
            invoke_function_store_async("get_storage", Some(delay_clone.clone()), Some(StorageEntry::new_delay(String::new()))).await;
        });
        
        ||{}
    });
    let clone_list = record_list.clone();
    let on_submit = Callback::from(move |record: Record| {
        let record = record.clone();
        let clone_list = clone_list.clone();
        spawn_local(async move{
            invoke_function_async("add_record", None, Some(Record { uuid: "".to_string(), ..record})).await;
            invoke_function_vec_async("get_all_records", Some(clone_list.clone()), None).await;
            ()
        });
    });

    let clone_list = record_list.clone();
    let delete_handler = Callback::from(move |record: Record|{
        let record = record.clone();
        let clone_list = clone_list.clone();
        spawn_local(async move {
            invoke_function_async("delete_record", None, Some(record)).await;
            invoke_function_vec_async("get_all_records", Some(clone_list.clone()), None).await;
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
            invoke_function_vec_async("get_all_records", Some(clone_list.clone()), None).await;
            ()
        });
    });

    
    let delay_clone = delay.clone();
    let settings_handler = Callback::from(move |input: String|{
        if input.contains("#") {
            change_background(&input);
        } else {
            invoke_function_store("store_storage", None, Some(StorageEntry::new_delay(input.clone())));
            delay_clone.set(StorageEntry::new_delay(NormalizeDelay::convert_to_string(input)));
        }
    });
    
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
                    <RecordList list = {(*record_list).clone()} delete_callback = {delete_handler} edit_callback = {edit_handler}/>
                </div>
            </div>
            <Settings callback={settings_handler} delay={(*delay).clone()} />
        </div>
    }
}
