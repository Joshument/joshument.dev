mod gif_img;
mod route;
mod theme_toggle;

use yew::prelude::*;
use yew_router::prelude::*;
use route::*;
use gif_img::GifImg;
use theme_toggle::ThemeToggle;

#[function_component(App)]
fn app() -> Html {
    html! {
        <body id="root">
            <header>
                <h1>{"joshument.dev"}</h1>
                <nav>
                    <ul>
                        <li>
                            <a href="/">{"home"}</a>
                            <div></div>
                        </li>
                        <li>
                            <a href="about">{"about"}</a>
                            <div></div>
                        </li>
                        <li>
                            <a href="projects">{"projects"}</a>
                            <div></div>
                        </li>
                        <li>
                            <a href="refsheet">{"reference sheet"}</a>
                            <div></div>
                        </li>
                    </ul>
                </nav>
                <div class="right-buttons">
                    <ThemeToggle/>
                </div>
            </header>

            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)}/>
            </BrowserRouter>


            <footer>
                {"This website is open source! Feel free to check out the "}
                <a href="https://github.com/Joshument/joshument.dev">{"source code"}</a>
            </footer>

            <div class="lily">
                <GifImg gif="img/movinglily.gif" img="img/lily.png" alt="lily" gifalt="lily" width="48" height="48"/>
            </div>
        </body>
    }
}

fn main() {
    yew::start_app::<App>();
}