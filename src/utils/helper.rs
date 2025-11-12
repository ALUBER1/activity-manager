use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use shared::models::{dto::record_dto::RecordDto, record::Record, storage_entry::StorageEntry};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct Args {
    pub record: RecordDto,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ArgsStore {
    pub storageEntry: StorageEntry,
}

pub async fn invoke_function_async<'a>(
    function: &'a str,
    result: Option<Rc<UseStateHandle<Record>>>,
    args: Option<Record>,
) where
    'a: 'static,
{
    if args.is_none() {
        let buffer = invoke(function, JsValue::null()).await;
        if !result.is_none() {
            result
                .unwrap()
                .set(from_value(buffer).expect("wasn't able to extract value"));
        }
    } else {
        let buffer = invoke(
            function,
            to_value(&Args {
                record: RecordDto::from(args.unwrap()),
            })
            .unwrap(),
        )
        .await;
        if !result.is_none() {
            result
                .unwrap()
                .set(from_value(buffer).expect("wasn't able to extract value"));
        }
    }
}

pub fn invoke_function<'a>(
    function: &'a str,
    result: Option<Rc<UseStateHandle<Record>>>,
    args: Option<Record>,
) where
    'a: 'static,
{
    if args.is_none() {
        spawn_local(async {
            let buffer = invoke(function, JsValue::null()).await;
            if !result.is_none() {
                result
                    .unwrap()
                    .set(from_value(buffer).expect("wasn't able to extract value"));
            }
        });
    } else {
        spawn_local(async {
            let buffer = invoke(
                function,
                to_value(&Args {
                    record: RecordDto::from(args.unwrap()),
                })
                .unwrap(),
            )
            .await;
            if !result.is_none() {
                result
                    .unwrap()
                    .set(from_value(buffer).expect("wasn't able to extract value"));
            }
        })
    }
}

pub async fn invoke_function_vec<'a>(
    function: &'a str,
    result: Option<UseStateHandle<Vec<Record>>>,
    args: Option<Record>,
) where
    'a: 'static,
{
    if args.is_none() {
        let buffer = invoke(function, JsValue::null()).await;
        if !result.is_none() {
            result
                .unwrap()
                .set(from_value(buffer).expect("wasn't able to extract value"));
        }
    } else {
        let buffer = invoke(
            function,
            to_value(&Args {
                record: RecordDto::from(args.unwrap()),
            })
            .unwrap(),
        )
        .await;
        if !result.is_none() {
            result
                .unwrap()
                .set(from_value(buffer).expect("wasn't able to extract value"));
        }
    }
}

pub async fn invoke_function_store<'a>(
    function: &'a str,
    result: Option<UseStateHandle<StorageEntry>>,
    args: Option<StorageEntry>,
) where
    'a: 'static,
{
    if args.is_none() {
        let buffer = invoke(function, JsValue::null()).await;
        if !result.is_none() {
            result
                .unwrap()
                .set(from_value(buffer).expect("wasn't able to extract value"));
        }
    } else {
        let buffer = invoke(
            function,
            to_value(&ArgsStore {
                storageEntry: args.unwrap(),
            })
            .unwrap(),
        )
        .await;
        if !result.is_none() {
            let value = from_value::<StorageEntry>(buffer).unwrap();
            result.unwrap().set(value);
        }
    }
}
