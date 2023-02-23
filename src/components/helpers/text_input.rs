use yew::prelude::*;

#[derive(Properties, PartialEq)] // uses yew properties trait
pub struct Props {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    html! {
        <div>
            <input type="text" name={props.first_name.clone()} placeholder="First Name" />
            <input type="text" name={props.last_name.clone()}  placeholder="Last Name" />
            <input type="text" name={props.email.clone()} placeholder="Email" />
        </div>
    }
}