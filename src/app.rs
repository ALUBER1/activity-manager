use yew::prelude::*;
use crate::{components::molecules::{form::{Event, Form}, record_list::{Record, RecordList}}, helper::{self, invoke_function, invoke_function_vec}};

#[function_component(App)]
pub fn app() -> Html {
    
    let record_list: UseStateHandle<Vec<Record>> = use_state(||Vec::new());
    let clone_list = record_list.clone();
    use_effect_with((), |_|{
        invoke_function("create_database", None, None);
        invoke_function("initialize_database", None, None);
        invoke_function_vec("get_all_records", Some(clone_list), None);
        
    });
    let clone_list = record_list.clone();
    let on_submit = 
        Callback::from(move |data: Event| {
            let clone_list = clone_list.clone();
            invoke_function("add_record", None, Some(helper::Record {
                name: data.name.clone(),
                date: data.date.clone(),
                time: data.time.clone(),
            }));
            
            invoke_function_vec("get_all_records", Some(clone_list), None);
        });
    
        

    html! {
        <div>
            <div>
                <Form on_submit = {on_submit}/>
            </div>
            <div>
                <RecordList list = {(*record_list).clone()} />
            </div>
        </div>
    }
}
