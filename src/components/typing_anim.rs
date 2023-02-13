use std::{time, rc::Rc};

use gloo::{timers::callback::Interval, console};
use yew::{Component, Properties, Context, Html, html, html::IntoPropValue, Classes, classes, callback::Callback, function_component, use_state, UseStateHandle, use_effect_with_deps, use_effect, use_callback, Reducible, use_reducer};

/// Properties for the TypingAnim component.
#[derive(Properties, PartialEq)]
pub struct TypingAnimProps {
    /// Forwards the html `class` parameter seen on other elements.
    #[prop_or_default]
    pub class: Classes,
    /// The text to display with the component.
    #[prop_or_default]
    pub text: String,
    /// The interval of which to type a character.
    #[prop_or(time::Duration::from_secs(1))]
    pub interval: time::Duration,
    /// Callback to call when the animation is finished.
    #[prop_or_default]
    pub on_finish: Callback<()>,
    /// Callback to call when the animation is finished and you want to propagate to other elements.
    #[prop_or_default]
    pub on_finish_propagate: Callback<()>
}

#[derive(Default)]
pub struct TypingAnimState {
    pub index: usize,
    pub text: String,
    pub class: Classes,
    pub interval: time::Duration,
    pub on_finish_propagate: Callback<()>,
}

pub enum TypingAnimAction {
    Tick
}

impl Reducible for TypingAnimState {
    type Action = TypingAnimAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_ctx: Self = (*self).clone();
        
        match action {
            Self::Action::Tick => {
                if next_ctx.index + 1 < next_ctx.text.len() {
                    next_ctx.index += 1;
                } else {
                    next_ctx.class.push(classes!("after:animate-caret-cursor"));
                    // next_ctx.on_finish.emit();
                    next_ctx.on_finish_propagate.emit();
                };
            }
        }

        next_ctx.into()
    }
}

#[function_component(TypingAnim)]
pub fn typing_anim(TypingAnimProps{
    class, text, interval, on_finish, on_finish_propagate
}: &TypingAnimProps) -> Html {
    let ctx = use_reducer(|| {
        TypingAnimState {
            interval: ,
            index: 0,
            text: text.clone(),
            on_finish: on_finish.clone(),
            on_finish_propagate: on_finish_propagate.clone()
        }
    });

    // Only invoke the call once
    use_effect_with_deps(move |_| {
        let interval = Interval::new(
            interval.as_millis().try_into().expect("Interval too large!"),
            move || {
                callback.dispatch(TypingAnimAction::Tick)
            }
        ),
        // Cleanup function
        move || interval.cancel()
    }, ());

    let class = callback.class.clone();
    let text = callback.text.clone();

    html! {
        <p class={class.clone()}>{ctx.text[0..ctx.index].clone()}</p>
    }
}
