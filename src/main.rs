mod components;
mod pages;
mod router;







use yew;

use yew::prelude::*;
use yew_router::prelude::*;



#[function_component]
fn App() -> Html {
    html!(
        <div id={"root"} class={classes!("dark")}>
            <body class={classes!("font-mono", "bg-slate-200", "dark:bg-slate-800", "w-full", "h-screen")}>
                <BrowserRouter>
                    <Switch<router::Route> render={router::switch}/>
                </BrowserRouter>
            </body>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
