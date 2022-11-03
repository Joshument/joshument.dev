use wasm_bindgen::JsCast;
use yew::prelude::*;
use yewdux::{prelude::*, dispatch};
use serde::{Serialize, Deserialize};
use web_sys::{window, HtmlDocument};

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Store)]
#[store(storage = "local", storage_tab_sync)]
struct ThemeCache {
    is_dark: bool,
}

impl Default for ThemeCache {
    fn default() -> Self {
        let window = window()
            .expect("failed to get window!");

        let is_dark = window.match_media("(prefers-color-scheme: dark)")
            .expect("failed to get media query!")
            .expect("media query does not exist!")
            .matches();

        Self {
            is_dark
        }
    }
}

pub enum Msg {
    ToggleTheme
}

pub struct ThemeToggle {
    text: String,
    is_dark: bool
}

impl Component for ThemeToggle {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = window()
            .expect("failed to get window!");

        let document = window.document()
            .expect("failed to get document!");

        let html_document = document.dyn_into::<HtmlDocument>()
            .expect("failed to convert document to HtmlDocument!");

        let cookies = html_document.cookie().expect("could not get cookies!");
        web_sys::console::log_1(&cookies.clone().into());
        let is_dark = cookies.find("dark").is_some();
        web_sys::console::log_1(&is_dark.into());

        let mut component = Self {
            text: match is_dark {
                true => String::from("switch to light mode"),
                false => String::from("switch to dark mode")
            },
            is_dark
        };

        component.set();
        component
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTheme => {
                self.is_dark = !self.is_dark;

                self.set();

                let document = window()
                    .expect("failed to get window!")
                    .document()
                    .expect("failed to get document!");

                let html_document = document.dyn_into::<HtmlDocument>()
                    .expect("failed to convert document to HtmlDocument!");
        
                html_document.set_cookie(match self.is_dark {
                    true => "dark",
                    false => "default"
                }).expect("could not set cookie!");
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::ToggleTheme);
        html!{
            <button type="button" {onclick}>
                {self.text.clone()}
            </button>
        }
    }
}

impl ThemeToggle {
    fn set(&mut self) {
        self.text = match self.is_dark {
            true => String::from("switch to light mode"),
            false => String::from("switch to dark mode")
        };

        let document = window()
            .expect("failed to get window!")
            .document()
            .expect("failed to get document!");

        document
            .get_element_by_id("root")
            .expect("failed to get #root element!")
            .set_attribute(
                "class", 
                match self.is_dark {
                    true => "dark-theme",
                    false => "default-theme"
                }
            )
            .expect("failed to set element attribute!");
    }
}