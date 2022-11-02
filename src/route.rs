mod about;
mod home;
mod projects;
mod refsheet;

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
        Route::About => html! {
            <div class="content">
                <p>{"about"}</p>
            </div>
        },
        Route::Projects => html! {
            <div class="content">
                <p>{"projects"}</p>
            </div>
        },
        Route::RefSheet => html! {
            <div class="content">
                <p>{"refsheet"}</p>
            </div>
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}