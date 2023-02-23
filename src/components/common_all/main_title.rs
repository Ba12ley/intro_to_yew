use stylist::style;
use yew::prelude::*;

// To add properties to a component, you need to add a struct that implements the properties trait
#[derive(Properties, PartialEq)] // uses yew properties trait
pub struct Props {
    pub title: String,
    pub color: Color,
}
#[derive(PartialEq)]
pub enum Color {
    Latest,
    Reed,
    Old,
}

impl Color{
    pub fn to_string(&self) -> String {
        match self {
            Color::Latest => "latest".to_owned(),
            Color::Reed => "reed".to_owned(),
            Color::Old => "old".to_owned(),
        }
    }
}

#[function_component(MainTitle)] // this is a macro that converts the function into a component
pub fn main_title(props: &Props) -> Html {
    let stylesheet = style!(r#"
        .latest {
            color: green;
            }
        .reed {
            color: grey;}
        .old {
            color: red;
            }
        "#).unwrap();
    html! {
        <div class={stylesheet}>
            <h1 class={props.color.to_string()}>{&props.title}</h1>
        </div>
    }
}