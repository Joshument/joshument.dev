use std::{iter::Iterator, borrow::Borrow, mem, time, cell::RefCell, rc::Rc, ops::Deref};

use gloo::{timers::callback::Interval, console};
use yew::{AttrValue, Component, Properties, Context, Html, html, html::IntoPropValue, Classes, classes, callback::Callback, function_component, use_state, UseStateHandle, use_effect_with_deps, use_effect, use_callback, Reducible, use_reducer};

type OptionalRefCellStateHandle<T> = Option<UseStateHandle<RefCell<T>>>;

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
    #[prop_or(Callback::from(|_| ()))]
    pub on_finish: Callback<Rc<TypingAnimState>, ()>,
    /// Callback to call when the animation is finished and you want to propagate to other elements.
    #[prop_or(Callback::from(|_| ()))]
    pub on_finish_propagate: Callback<(), ()>
}

pub struct TypingAnimState {
    pub interval: RefCell<Option<Interval>>,
    chars: OptionalRefCellStateHandle<String>,
    pub text: OptionalRefCellStateHandle<String>,
    pub class: OptionalRefCellStateHandle<Classes>,
    /// Callback to call when the animation is finished.
    on_finish: Callback<Rc<TypingAnimState>, ()>,
    /// Callback to call when the animation is finished and you want to propagate to other elements.
    on_finish_propagate: Callback<(), ()>
}

pub enum TypingAnimAction {
    Tick
}

impl Default for TypingAnimState {
    fn default() -> Self {
        Self {
            interval: None.into(),
            chars: None,
            text: None,
            class: None,
            on_finish: Callback::from(|_| ()),
            on_finish_propagate: Callback::from(|_| ())
        }
    }
}

impl Reducible for TypingAnimState {
    type Action = TypingAnimAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Self::Action::Tick => {
                let char = self.chars.as_ref().unwrap().borrow_mut().pop();
                if let Some(char) = char {
                    self.text.as_ref().unwrap().borrow_mut().push(char);
                } else {
                    self.interval.borrow_mut().take().unwrap().cancel();
                    self.class.as_ref().unwrap().borrow_mut().push(classes!("after:animate-caret-cursor"));

                    // self.on_finish.emit(self.clone());
                    self.on_finish_propagate.emit(());
                };
            }
        }

        self
    }
}

#[function_component(TypingAnim)]
pub fn typing_anim(props: &TypingAnimProps) -> Html {
    let class = use_state(|| RefCell::new(props.class.clone()));
    let chars: UseStateHandle<RefCell<String>> = use_state(|| RefCell::new(props.text.clone().chars().rev().collect()));
    let text = use_state(|| RefCell::new(String::new()));

    let callback = use_reducer(|| TypingAnimState {
        interval: None.into(),
        chars: Some(chars),
        text: Some(text),
        class: Some(class),
        on_finish: props.on_finish.clone(),
        on_finish_propagate: props.on_finish_propagate.clone()
    });

    let interval = {
        let callback = callback.clone();
        Interval::new(
            props.interval.as_millis().try_into().expect("Interval too large!"), 
            move || {
                callback.dispatch(TypingAnimAction::Tick)
            }
        )
    };

    callback.interval.replace(Some(interval));

    let class = callback.class.clone();
    let text = callback.text.clone();

    html! {
        <p class={class.unwrap().deref().borrow().clone()}>{text.unwrap().deref().borrow().clone()}</p>
    }
}