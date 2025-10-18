use yew::prelude::*;
use crate::{components::molecules::{form::Form, record_list::RecordList, title_bar::TitleBar, settings::Settings}, functions::Functions, helper::{invoke_function, invoke_function_vec}};
use shared::models::record::Record;

#[function_component(App)]
pub fn app() -> Html {
    
    let record_list: UseStateHandle<Vec<Record>> = use_state(||Vec::new());
    use_effect_with((), |_|{
        invoke_function("create_database", None, None);
        invoke_function("initialize_database", None, None);
        ||{}
    });
    let clone_list = record_list.clone();
    let on_submit = 
        Callback::from(move |data: Record| {
            invoke_function("add_record", None, Some(Record { uuid: "".to_string(), name: data.name, date: data.date, time: data.time}));
        });

    let remove_handler = Callback::from(move |record: Record|{
        invoke_function("delete_record", None, Some(Record::from(record)));
    });

    let title_handler = Callback::from(move |function: Functions| {
        match function {
            Functions::Close => invoke_function("close_app", None, None),
            Functions::Minimize => invoke_function("minimize_app", None, None),
            Functions::Maximize => invoke_function("maximize_app", None, None)
        }
    });
    
    invoke_function_vec("get_all_records", Some(clone_list), None);

    html! {
        <div id="main">
            <div id="fixed">
                <TitleBar on_click={title_handler}></TitleBar>
                <div id="form">
                    <Form on_submit = {on_submit}/>
                </div>
                <Settings />
            </div>
            <div id="non-fixed">
                <div id="record-list">
                    <RecordList list = {(*record_list).clone()} callback = {remove_handler} />
                </div>
            </div>
        </div>
    }
}
