use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::UseStateHandle;
use shared::models::record::Record;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct Args{
    pub record: Record
}

pub async fn invoke_function_async<'a>(function: &'a str, result: Option<Rc<UseStateHandle<Record>>>, args: Option<Record>) where 'a:'static{
    if args.is_none(){
        let buffer = invoke(function, JsValue::null()).await;
        if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
    } else {
        let buffer = invoke(function, to_value(&Args{record: args.unwrap()}).unwrap()).await;
        if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
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

pub async fn invoke_function_vec_async<'a>(function: &'a str, result: Option<UseStateHandle<Vec<Record>>>, args: Option<Record>) where 'a:'static{
    if args.is_none(){
        let buffer = invoke(function, JsValue::null()).await;
        if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
    } else {
        let buffer = invoke(function, to_value(&Args{record: Record::from(args.unwrap())}).unwrap()).await;
        if !result.is_none() {result.unwrap().set(from_value(buffer).expect("wasn't able to extract value"));}
    }
}

pub fn invoke_function_vec<'a>(function: &'a str, result: Option<UseStateHandle<Vec<Record>>>, args: Option<Record>) where 'a:'static{
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