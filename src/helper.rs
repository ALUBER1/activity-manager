use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::{Properties, UseStateHandle};
use components::molecules::record_list::Record as RecordOut;

use crate::components;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize, Clone, Properties, PartialEq)]
pub struct Record{
    pub name: String,
    pub date: String,
    pub time: String
}

#[derive(Serialize, Deserialize)]
pub struct Args{
    pub record: Record
}

impl Record {
    pub fn default() -> Record {
        Record { name: "".to_string(), date: "".to_string(), time: "".to_string() }
    }

    pub fn record_by_name(name: String) -> Record {
        Record{name, date: "".to_string(), time: "".to_string() }
    }

    pub fn from(other: RecordOut) -> Record{
        Record{name: other.name, date: other.date, time: other.time}
    }
}

pub fn invoke_function<'a>(function: &'a str, result: Option<Rc<UseStateHandle<Record>>>, args: Option<Record>) where 'a:'static{
    if args.is_none(){
        spawn_local(async{
            let buffer = invoke(function, JsValue::null()).await;
            if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
        });
    } else {
        spawn_local(async{
            let buffer = invoke(function, to_value(&Args{record: args.unwrap()}).unwrap()).await;
            if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
        })
    }
}

pub fn invoke_function_vec<'a>(function: &'a str, result: Option<UseStateHandle<Vec<RecordOut>>>, args: Option<RecordOut>) where 'a:'static{
    if args.is_none(){
        spawn_local(async {
            let buffer = invoke(function, JsValue::null()).await;
            
            if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
        });
    } else {
        spawn_local(async {
            let buffer = invoke(function, to_value(&Args{record: Record::from(args.unwrap())}).unwrap()).await;
            if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
        })
    }
}