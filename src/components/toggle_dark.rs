use yew::prelude::*;
use gloo::utils::document;

#[function_component(ToggleDark)]
pub fn toggle_dark() -> Html {
    let onclick = {
        Callback::from(move |_| {
            let root = document()
                .get_element_by_id("root")
                .expect("Could not get root node! Did you change the id?");
            let classes = root.class_list();
            
            classes.toggle("dark").expect("Failed to toggle dark mode!");
        })
    };

    html! {
        <button {onclick} type={"button"} class={classes!("bg-fuchsia-500")}>{"Toggle Dark Mode"}</button>
    }
}