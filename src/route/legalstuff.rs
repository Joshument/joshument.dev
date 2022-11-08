use yew::prelude::*;
use crate::local_time;

pub fn page() -> Html {
    html! { 
        <div class="content">
            <h1>{"Legal Stuff"}</h1>
            <h2>{"99% of you probably don't care about this but I abide by the law"}</h2>
            <p>{"This website uses the "}<a href="https://github.com/tonsky/FiraCode">{"Fira Code"}</a>{" font."}</p>
        </div>
    }
}