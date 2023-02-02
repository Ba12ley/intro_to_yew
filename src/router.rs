use yew_router::prelude::*;
use yew::prelude::*;
#[derive(Switch, Debug, Clone,PartialEq, Routable)]
pub enum Route{
    #[at("/")]
    Home,
    About,
    Contact,
    Blog,
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html!{"Home"},
        Route::About => html!{"About"},
        Route::Contact => html!{"Contact"},
        Route::Blog => html!{"Blog"},
        Route::NotFound => html!{"404"},
    }
}