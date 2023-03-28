use yew::prelude::*;

use crate::components::typing_anim::TypingAnim;

pub fn page() -> Html {
    html! {
        <div id={"content"} class={classes!("flex", "items-end", "flex-col", "h-screen")}>
            <TypingAnim
                class={classes!("flex", "shrink", "mb-0", "text-fuchsia-500", r"after:content-['\_']", "m-auto", "text-3xl")}
                text={"404"}
                interval={std::time::Duration::from_millis(100)}
            />
            <TypingAnim
                class={classes!("flex", "mt-0", "text-fuchsia-500", r"after:content-['\_']", "m-auto", "text-base")}
                text={"page not found"}
                interval={std::time::Duration::from_millis(50)}
            />
        </div>
    }
}
