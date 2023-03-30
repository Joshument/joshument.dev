use yew::prelude::*;

use crate::components::typing_anim::TypingAnim;

#[function_component(Page)]
pub fn page() -> Html {
    html! {
        <div id={"content"} class={classes!("flex", "items-end", "flex-col", "h-screen")}>
            <TypingAnim
                class={classes!("flex", "shrink", "text-fuchsia-500", r"after:content-['\_']", "m-auto", "text-9xl")}
                text={"404"}
                interval={std::time::Duration::from_millis(100)}
            />
        </div>
    }
}
