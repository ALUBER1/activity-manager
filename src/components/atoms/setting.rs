use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub children: Children
}

#[function_component(Setting)]
pub fn setting(prop: &Props) -> Html {
    html!{
        <>
            <div class="setting">
                <p class="setting-label">{prop.label.clone()}</p>
                {prop.children.clone()}
            </div>
            <hr class="settings-divisor"/>
        </>
    }
}