use std::{borrow::Borrow, ops::Deref};

use gloo::console;
use yew::prelude::*;

use crate::components::typing_anim::TypingAnim;
use crate::components::page_body::PageBody;

pub fn page() -> Html {
    html!{
        <body id="pagebody" class={classes!("flex", "items-center", "justify-between", "flex-col", "h-screen")}>
            <TypingAnim 
                class={classes!(
                    "transition-all", "duration-700", "mb-0", "mt-52",
                    "text-fuchsia-500", r"after:content-['\_']", "m-auto", "text-3xl"
                )} 
                text={"joshument.dev"} 
                interval={std::time::Duration::from_millis(100)}
                on_finish={Callback::from(|mut component: TypingAnim| {
                    
                    console::log!(format!("{:?}", component.class));
                    component.class = component
                        .class
                        .into_iter()
                        .filter(|x| {
                            x.deref() != "mt-52"
                        })
                        .collect();
                    // component.class.push("mb-auto");
                    component.class.push("mt-5");
                    console::log!(format!("{:?}", component.class));
                    component
                })}
            />
            <TypingAnim 
                class={classes!("my-5", "text-zinc-800", "dark:text-zinc-300", r"after:content-['\_']", "m-auto", "mx-3", "text-base")} 
                text={"Mark: Hello everybody, my name is Markiplier and welcome to Five Nights at Freddy's, an indie horror game that you guys suggested, in mass, and I saw that Yamimash played it and he said it was really really good... So I'm very eager to see what is up. And that is a terrifying animatronic bear! \"Family pizzeria looking for security guard to work the nightshift.\" Oh...12 a.m. The first night. If I didn't wanna stay the first night, why would I stay any more than... five... Why I stay any more than two- hello? Okay..."} 
                interval={std::time::Duration::from_millis(15)}
            />
            <TypingAnim 
                class={classes!("mt-0", "text-fuchsia-500", r"after:content-['\_']", "m-auto", "text-base")} 
                text={"under construction™"} 
                interval={std::time::Duration::from_millis(50)}
            />
        </body>
    }
}