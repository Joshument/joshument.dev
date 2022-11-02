mod about;
mod home;
mod projects;
mod refsheet;
mod notfound;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/projects")]
    Projects,
    #[at("/refsheet")]
    RefSheet,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => home::page(),
        Route::About => about::page(),
        Route::Projects => projects::page(),
        Route::RefSheet => refsheet::page(),
        Route::NotFound => notfound::page(),
    }
}