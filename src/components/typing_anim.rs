use std::{borrow::BorrowMut, cell::RefCell, iter::Iterator, ops::Deref, time};

use gloo::timers::callback::Interval;
use yew::{
    callback::Callback, classes, function_component, html, use_reducer, use_state, AttrValue,
    Classes, Html, Properties, Reducible, UseStateHandle,
};

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
    pub on_finish: Callback<(), ()>,
}

pub struct TypingAnimState {
    pub interval: RefCell<Option<Interval>>,
    chars: OptionalRefCellStateHandle<String>,
    pub text: OptionalRefCellStateHandle<String>,
    pub class: RefCell<Classes>,
    /// Callback to call when the animation is finished. Does not apply to inner elements.
    on_finish: Callback<(), ()>,
}

pub enum TypingAnimAction {
    Tick,
}

impl Default for TypingAnimState {
    fn default() -> Self {
        Self {
            interval: None.into(),
            chars: None,
            text: None,
            class: RefCell::new(classes!()),
            on_finish: Callback::from(|_| ()),
        }
    }
}

impl Reducible for TypingAnimState {
    type Action = TypingAnimAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            Self::Action::Tick => {
                let mut class = self.class.borrow_mut();
                let mut text = self.text.as_ref().unwrap().deref().borrow_mut();
                //let char = self.chars.as_ref().unwrap().borrow_mut().deref().deref().deref().borrow_mut().pop();
                let char = self.chars.as_ref().unwrap().deref().borrow_mut().pop();

                if let Some(char) = char {
                    text.push(char);
                } else {
                    if !class.contains("finished-anim") {
                        self.on_finish.emit(())
                    }

                    self.interval.borrow_mut().take().unwrap().cancel();
                    class.push(classes!("after:animate-caret-cursor", "finished-anim"));
                };
            }
        }

        self
    }
}

#[function_component(TypingAnim)]
pub fn typing_anim(props: &TypingAnimProps) -> Html {
    let chars: UseStateHandle<RefCell<String>> =
        use_state(|| RefCell::new(props.text.clone().chars().rev().collect()));
    let text = use_state(|| RefCell::new(String::new()));

    let callback = use_reducer(|| TypingAnimState {
        interval: None.into(),
        chars: Some(chars),
        text: Some(text),
        class: RefCell::new(classes!()),
        on_finish: props.on_finish.clone(),
    });

    let interval = {
        let callback = callback.clone();
        Interval::new(
            props
                .interval
                .as_millis()
                .try_into()
                .expect("Interval too large!"),
            move || callback.dispatch(TypingAnimAction::Tick),
        )
    };

    callback.interval.replace(Some(interval));

    let class = callback.class.borrow_mut().clone();
    let text = callback
        .text
        .as_ref()
        .unwrap()
        .deref()
        .clone()
        .borrow()
        .clone();

    html! {
        <p class={classes![props.class.clone(), class]}>{text}</p>
    }
}
