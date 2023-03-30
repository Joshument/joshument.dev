use core::time;

use std::mem;



use gloo::timers::callback::Timeout;
use yew::prelude::*;


use crate::components::typing_anim::{TypingAnim};

const INTRO_LEN: usize = 1_500;
const TITLE: &str = "joshument.dev";
const SUBTITLE: &str = "remind me to never write an intro animation again";

#[function_component(Page)]
pub fn page() -> Html {
    let anim_finished = use_state(|| false);
    let finish_anim = {
        let anim_finished = anim_finished.clone();
        Callback::from(move |_| {
            let anim_finished = anim_finished.clone();
            let timeout = Timeout::new(500, move || anim_finished.set(true));
            mem::forget(timeout);
        })
    };

    html! {
        <body id="pagebody" class={classes!("flex", "items-center", "justify-between", "flex-col", "h-screen")}>
            <TypingAnim
                class={classes!(
                    "transition-all", "duration-700", if *anim_finished {"mt-2"} else {"mt-[50vh]"},
                    "text-fuchsia-500", r"after:content-['\_']", "text-3xl"
                )}
                text={TITLE}
                interval={time::Duration::from_millis((INTRO_LEN / TITLE.len()).try_into().unwrap())}
                on_finish={finish_anim.clone()}
            />
            <TypingAnim
                class={classes!(
                    "transition-all", "duration-700", if *anim_finished {"mb-2"} else {"mb-[50vh]"},
                    "text-fuchsia-500", r"after:content-['\_']", "text-1xl"
                )}
                text={SUBTITLE}
                interval={time::Duration::from_millis((INTRO_LEN / SUBTITLE.len()).try_into().unwrap())}
                on_finish={finish_anim.clone()}
            />
        </body>
    }
}
