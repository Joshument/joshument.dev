use std::{iter::Iterator, borrow::Borrow, mem, time};

use gloo::{timers::callback::Interval, console};
use yew::{AttrValue, Component, Properties, Context, Html, html, html::IntoPropValue, Classes, classes, callback::Callback};

/// Properties for the TypingAnim component.
#[derive(Properties, PartialEq)]
pub struct TypingAnimProps {
    /// Forwards the html `class` parameter seen on other elements.
    #[prop_or_default]
    pub class: Classes,
    /// The text to display with the component.
    #[prop_or_default]
    pub text: AttrValue,
    /// The interval of which to type a character.
    #[prop_or(time::Duration::from_secs(1))]
    pub interval: time::Duration,
    /// Callback to call when the animation is finished.
    #[prop_or(Callback::from(|component: TypingAnim| component))]
    pub on_finish: Callback<TypingAnim, TypingAnim>,
    /// Callback to call when the animation is finished and you want to propagate to other elements.
    #[prop_or(Callback::from(|_| ()))]
    pub on_finish_propagate: Callback<(), ()>
}

/// Events for the TypingAnim component.
pub enum TypingAnimMessage {
    /// Tick the typing. This is what causes a character to be typed, and is mostly used internally.
    Tick,
}

/// Component used to simulate a "typing" effect, where the typing is done when the component is loaded.
/// 
/// This component also allows you to have a "blinking" animation on the caret cursor if you specify it with ::after in css.
pub struct TypingAnim {
    /// The text that gets displayed on the DOM. 
    /// This is a `String` so that it can be edited without large amounts of cloning or reallocation.
    text: String,
    /// The characters that are in queue to be put on to `text`.
    /// This is a vector so that it can be sized. It is a requirement for all component structs to be sized.
    chars: Vec<char>,
    /// Interally used to store the JavaScript interval for the typing.
    /// This is in the struct properties so that it can be cancelled from anywhere.
    pub interval: Option<Interval>,
    /// The classes that are displayed.
    /// This differs from the `class` property in that certain classes are added under certain conditions.
    pub class: Classes
}

impl Default for TypingAnim {
    fn default() -> Self {
        Self {
            text: String::new(),
            chars: vec![],
            interval: None,
            class: classes!()
        }
    }
}

impl Component for TypingAnim {
    type Message = TypingAnimMessage;
    type Properties = TypingAnimProps;

    fn create(ctx: &Context<Self>) -> Self {
        console::log!("hell");
        Self {
            text: "".into(),
            chars: ctx.props().text.clone().chars().rev().collect(),
            interval: None,
            class: ctx.props().class.clone()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TypingAnimMessage::Tick => {
                let char = self.chars.pop();
                let char = if let Some(char) = char {
                    char
                } else {
                    self.interval.take().unwrap().cancel();
                    self.class.push(classes!("after:animate-caret-cursor"));

                    *self = ctx.props().on_finish.emit(mem::take(self));
                    ctx.props().on_finish_propagate.emit(());

                    return true
                };

                self.text.push(char);
                true
            },

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p class={self.class.clone()}>{&self.text}</p>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let interval = ctx.props().interval;

            let callback = ctx.link().callback(|_| TypingAnimMessage::Tick);
            self.interval = Some(Interval::new(
                interval.as_millis().try_into().expect("Interval too large!"),
                move || {
                    callback.emit(())
                }
            ));
        }
    }
}