use core::time;
use std::{borrow::Borrow, ops::Deref};

use gloo::console;
use yew::prelude::*;

use crate::components::page_body::PageBody;
use crate::components::typing_anim::{TypingAnim, TypingAnimState};

pub fn page() -> Html {
    html! {
        <body id="pagebody" class={classes!("flex", "items-center", "justify-between", "flex-col", "h-screen")}>
            <TypingAnim
                class={classes!(
                    "transition-all", "duration-700",
                    "text-fuchsia-500", r"after:content-['\_']", "m-auto", "text-3xl"
                )}
                text={"hi"}
                interval={time::Duration::from_millis(500)}
                on_finish={Callback::from(|_| console::log!("test"))}
            />
        </body>
    }
}