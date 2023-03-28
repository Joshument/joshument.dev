use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::home::page(),
        Route::NotFound => pages::notfound::page(),
    }
}
