use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,

}

#[function_component(Buttons)]
pub fn buttons(props : &Props) -> Html {
    html! {
        <div>
            <button>{ &props.label }</button>
        </div>
    }
}