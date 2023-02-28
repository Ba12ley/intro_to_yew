use yew::prelude::*;
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component};

mod components;
use components::common_all::main_title::{MainTitle, Color};
use components::page_components::form::Form;


#[derive(Serialize,Deserialize)]
struct BlogPost {
    title: String,
    body: String,
}



#[function_component(App)] // this is a macro that converts the function into a component
// #[styled_component(App)] // this is a macro that converts the function into a component
pub fn app() -> Html {

    let log_test = "Logged to console";
    let blog = BlogPost {
        title: "Hello World".to_owned(), // to_owned() converts a string literal to a string, where as to_string() converts a string to a string
        body: "This is a blog post".to_owned(),
    };
    let list_of_links = vec!["Link 1", "Link 2", "Link 3", "Link 4"];

    log!(log_test);
    // log!(blog.title);
    // log!(blog.body);
   // log!(serde_json::to_string_pretty(&blog).unwrap());
    let html_class_var = "variable_class";

    let main_title_onload = Callback::from(|message: String| log!(message));
    html! { // This is a macro that converts the html into a virtual dom
        <> // This is a fragment, fragments are used to return multiple elements

        <div class="can_add_class">{ "Can add class" }</div>
        <div class={html_class_var}>{ "Can use variable in attribute" }</div>
        <div>
            <MainTitle title="Properties added with Enum to Main Title" color={Color::Latest} onload={main_title_onload}/>
        <p>{"Main Title used from a component"}</p>
            <h1 style="color: red">{ "Hello World!" }</h1>
        </div>
        <div>
        <h2>{ "For Loop" }</h2>
            <ul style="color: blue">
                                                // || is a closure, it is a function that can be passed around
                { for list_of_links.iter().map(|link| html! { <li><a href="">{ link }</a></li> }) } //using iter() to iterate over the vector and not taking ownership of the vector.  using map to map the vector to html
            </ul>
        </div>
        <div>
            { "blogs" }
        </div>
        <div>
        <h2>{ "IF" }</h2>
        if &blog.title == "Hello World" {
                <div>
                    <p> {"if &blog.title == Hello World"} </p>
                    <h3>{ "Title: " }{ blog.title }</h3>
                </div>

        } else {

                <div>
                    <h3>{ "No Blog" }</h3>

                </div>

        }

        </div>
    <Form />
        </>
    }
}