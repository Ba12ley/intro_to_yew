use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub onclick: Callback<()>,
}

#[function_component(Buttons)]
pub fn buttons(props : &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_|{
        onclick.emit(());
    });
    html! {
        <div>
            <button onclick={button_onclick}>{ &props.label }</button>
        </div>
    }
}