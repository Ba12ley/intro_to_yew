use yew::prelude::*;
use crate::components::helpers::text_input::TextInput;
use crate::components::helpers::buttons::Buttons;


#[function_component(Form)]
pub fn form() -> Html {
    html! {
        <div>
            <h1>{ "Form" }</h1>
            <form>
                <TextInput first_name="first_name" last_name="last_name" email="email" />
                <Buttons label="Submit" />
            </form>
        </div>
    }
}