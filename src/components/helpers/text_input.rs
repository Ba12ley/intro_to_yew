use yew::prelude::*;
use gloo::console::log;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)] // uses yew properties trait
pub struct Props {
    pub first_name: String,
    pub handle_onchange: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_onchange = props.handle_onchange.clone();
    let onchange = Callback::from(move |event: Event| {
        // let target = event.target().unwrap();
        // let input = target.unchecked_into::<HtmlInputElement>();
        // log!(input.value()); // logs to the console, when the input is changed and clicking outside the field
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        // log!(value);
        handle_onchange.emit(value);


    });
    html! {
        <div>
            <input type="text" name={props.first_name.clone()} placeholder="First Name" onchange={onchange} />
            // <input type="text" name={props.last_name.clone()}  placeholder="Last Name" />
            // <input type="text" name={props.email.clone()} placeholder="Email" />
        </div>
    }
}