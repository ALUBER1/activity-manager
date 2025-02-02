use std::ops::Deref;
use yew::prelude::*;
use crate::components::atoms::{text_input::TextInput, button::Button};

#[derive(Default, Clone)]
pub struct Event {
    pub name: String,
    pub date: String,
    pub time: String
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<Event>
}

#[function_component(Form)]
pub fn form(props: &Props) -> Html{
    let value_state = use_state(||Event::default());

    let cloned_value = value_state.clone();
    let on_changename = Callback::from(move |name|{
        cloned_value.set(Event { name, ..cloned_value.deref().clone()});
    });

    let cloned_value = value_state.clone();
    let on_changedate = Callback::from(move |date|{
        cloned_value.set(Event { date,..cloned_value.deref().clone()});
    });

    let cloned_value = value_state.clone();
    let on_changetime = Callback::from(move |time|{
        cloned_value.set(Event { time,..cloned_value.deref().clone()});
    });

    let cloned_value = value_state.clone();
    let clone_submit = props.on_submit.clone();
    let on_submit = Callback::from(move |event: SubmitEvent|{
        event.prevent_default();
        let data = cloned_value.deref().clone();
        clone_submit.emit(data);
    });

    html!{
        <form onsubmit={on_submit}>
            <TextInput name="name" on_change={on_changename}/>
            <TextInput name="date" on_change={on_changedate}/>
            <TextInput name="time" on_change={on_changetime}/>
            <Button label = "submit"/>
        </form>
    }
}