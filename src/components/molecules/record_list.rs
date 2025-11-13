use i18nrs::yew::use_translation;
use yew::{function_component, html, use_state, Callback, Html, Properties};

use shared::models::record::Record;

use crate::{
    components::{atoms::record_button::RecordButton, molecules::edit_form::EditForm},
    errors::form_error::FormError,
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub list: Vec<Record>,
    pub delete_callback: Callback<Record>,
    pub edit_callback: Callback<Result<Record, Vec<FormError>>>,
}

#[function_component(RecordList)]
pub fn record_list(records: &Props) -> Html {
    let (i18n, _set_language) = use_translation();

    let editing = use_state(|| None::<Record>);

    let delete_handler = {
        let onclick = records.delete_callback.clone();
        Callback::from(move |a: Record| {
            onclick.emit(a);
        })
    };

    let edit_handler = {
        let editing_clone = editing.clone();
        Callback::from(move |a: Record| {
            editing_clone.set(Some(a));
        })
    };

    let cancel_handle = {
        let editing_clone = editing.clone();
        Callback::from(move |_| {
            editing_clone.set(None);
        })
    };

    let submit_handler = {
        let editing_clone = editing.clone();
        let onclick = records.edit_callback.clone();
        Callback::from(move |mut record: Result<Record, Vec<FormError>>| {
            if let Ok(record) = record.as_mut() {
                record.uuid = (*editing_clone).clone().unwrap().uuid;
                editing_clone.set(None);
            }
            onclick.emit(record);
        })
    };

    html! {
        <>
            if let Some(record) = (*editing).clone() {
                <div class="editing-panel">
                    <div class="editing-form-container">
                        <EditForm on_submit={submit_handler} record={record} cancel={cancel_handle} />
                    </div>
                </div>
            } else {
                <div id="record-list">
                    {
                        records.list.clone().into_iter().map(|element|{
                            html!{
                                <div class="record-list-style">
                                    <p class="record-label">
                                        {i18n.t("name")}
                                        {": "}
                                        {element.name.clone()}
                                        {", "}
                                        {i18n.t("date")}
                                        {": "}
                                        {element.get_date().clone()}
                                        {", "}
                                        {i18n.t("time")}
                                        {": "}
                                        {element.get_time().clone()}
                                    </p>
                                    <div class="record-button">
                                        <RecordButton id = {element.clone()}  onclick = {delete_handler.clone()} ty={"delete"}/>
                                        <RecordButton id = {element}  onclick = {edit_handler.clone()} ty={"edit"}/>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </>
    }
}
