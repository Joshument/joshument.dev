use std::{borrow::Borrow, ops::Deref};

use gloo::console;
use yew::prelude::*;

use crate::components::typing_anim::{TypingAnim, TypingAnimState};
use crate::components::page_body::PageBody;

pub fn page() -> Html {
    html!{
        <body id="pagebody" class={classes!("flex", "items-center", "justify-between", "flex-col", "h-screen")}>
            <p>{"damn!"}</p>
        </body>
    }
}