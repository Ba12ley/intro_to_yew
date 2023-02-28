use gloo::console::log;
use yew::prelude::*;
use yew::virtual_dom::ListenerKind::onchange;
use crate::components::helpers::text_input::TextInput;
use crate::components::helpers::buttons::Buttons;


#[function_component(Form)]
pub fn form() -> Html {
    let first_name_state = use_state(|| "no first name set".to_owned());
    let cloned_first_name_state = first_name_state.clone();
    let first_name_changed = Callback::from(move |first_name|{
        cloned_first_name_state.set(first_name);
        // log!("first name changed", &first_name);

    });
    let button_count_state = use_state(|| 0_u32);
    let cloned_button_count_state = button_count_state.clone();
    let button_clicked = Callback::from(move |_|{
        let count = *cloned_button_count_state;
        cloned_button_count_state.set(count + 1);
    });
    html! {
        <div>
            <h1>{ "Form" }</h1>
            <form>
                <TextInput first_name="first_name" handle_onchange={first_name_changed} />
                <p> { "First Name State: "}{&*first_name_state} </p> // * is dereference
                <Buttons label="Submit" onclick={button_clicked} />
                <p> {"Button has been clicked: "}{*button_count_state} </p>
            </form>
        </div>
    }
}