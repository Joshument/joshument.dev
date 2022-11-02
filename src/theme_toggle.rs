use yew::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
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

    fn create(_: &Context<Self>) -> Self {
        let window = window()
            .expect("failed to get window!");
        /* let document = window.document()
            .expect("failed to get document!");
        let html_document = document.dyn_into::<web_sys::HtmlDocument>()
            .expect("failed to get HTMLDocument!"); */

        let is_dark = window.match_media("(prefers-color-scheme: dark)")
            .expect("failed to get media query!")
            .expect("media query does not exist!")
            .matches();

        Self {
            text: match is_dark {
                true => String::from("switch to light mode"),
                false => String::from("switch to dark mode")
            },
            is_dark
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleTheme => {
                self.is_dark = !self.is_dark;

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