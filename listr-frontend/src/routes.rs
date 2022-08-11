use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::prelude::*;
use crate::components::list::list::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/lists")]
    Lists,
    #[at("/lists/:id")]
    List { id: i32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Lists => html! { <Lists/> },
        Route::List { id } => html! { <List id={*id} /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
