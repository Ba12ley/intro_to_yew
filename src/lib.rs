use yew::prelude::*;
use gloo::console::log;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct BlogPost {
    title: String,
    body: String,
}

#[function_component(App)] // this is a macro that converts the function into a component
pub fn app() -> Html {
    let log_test = "Logged to console";
    let blog = BlogPost {
        title: "Hello World".to_owned(), // to_owned() converts a string literal to a string, where as to_string() converts a string to a string
        body: "This is a blog post".to_owned(),
    };

    log!(log_test);
    // log!(blog.title);
    // log!(blog.body);
   // log!(serde_json::to_string_pretty(&blog).unwrap());
    let html_class_var = "variable_class";

    html! { // This is a macro that converts the html into a virtual dom
        <> // This is a fragment, fragments are used to return multiple elements
        <div class="can_add_class">{ "Can add class" }</div>
        <div class={html_class_var}>{ "Can use variable in attribute" }</div>
        <div>
            <h1 style="color: red">{ "Hello World!" }</h1>
        </div>
        <div>
            <ul style="color: blue">
                <li>{ "Link 1" }</li>
                <li>{ "Link 2" }</li>
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
        </>
    }
}