use std::ops::Deref;
use gloo::console::log;
use yew::prelude::*;
use yew::virtual_dom::ListenerKind::onchange;
use crate::components::helpers::text_input::TextInput;
use crate::components::helpers::buttons::Buttons;

#[derive(Default,Clone)]
struct Data {
    pub first_name: String,
    pub count: u32,
}

#[function_component(Form)]
pub fn form() -> Html {

    let state = use_state(|| Data::default());

    let cloned_state = state.clone();
    let first_name_changed = Callback::from(move |first_name|{
        let mut data = cloned_state.deref().clone(); // deref() dereferences the state, clone() clones the state, allows us to change the state
        data.first_name = first_name;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_|{
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data);
    });
    // let first_name_state = use_state(|| "no first name set".to_owned());
    // let cloned_first_name_state = first_name_state.clone();
    // let first_name_changed = Callback::from(move |first_name|{
    //     cloned_first_name_state.set(first_name);
        // log!("first name changed", &first_name);

    // });
    // let button_count_state = use_state(|| 0_u32);
    // let cloned_button_count_state = button_count_state.clone();
    // let button_clicked = Callback::from(move |_|{
    //     let count = *cloned_button_count_state;
    //     cloned_button_count_state.set(count + 1);
    // });
    html! {
        <div>
            <h1>{ "Form" }</h1>
            <form>
                <TextInput first_name="first_name" handle_onchange={first_name_changed} />
                <p> { "First Name State: "}{&state.first_name} </p> // * is dereference & is reference, by using both the trait Display is available
                // <p> { "First Name State: "}{&*first_name_state} </p> // * is dereference & is reference, by using both the trait Display is available
                <Buttons label="Submit" onclick={button_clicked} />
                <p> {"Button has been clicked: "}{state.count} </p>
                // <p> {"Button has been clicked: "}{*button_count_state} </p>
            </form>
        </div>
    }
}