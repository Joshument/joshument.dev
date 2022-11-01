use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1 class="title">{"joshument.dev"}</h1>
            <h2>{"Welcome to my website!"}</h2>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}