mod components;
mod pages;
mod router;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use components::typing_anim::{TypingAnim, TypingAnimState};
use gloo::console;
use yew;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew_router::prelude::*;

use components::toggle_dark::*;

#[function_component]
fn App() -> Html {
    html!(
        <div id={"root"} class={classes!("dark")}>
            <body class={classes!("font-mono", "bg-slate-200", "dark:bg-slate-800", "w-full", "h-screen")}>
                <BrowserRouter>
                    <Switch<router::Route> render={router::switch}/>
                </BrowserRouter>
            </body>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
