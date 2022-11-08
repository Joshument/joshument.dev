mod gif_img;
mod route;
mod theme_toggle;
mod local_time;

use yew::prelude::*;
use yew_router::prelude::*;
use route::*;
use gif_img::GifImg;
use theme_toggle::ThemeToggle;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <body id="root">
                <header>
                    <h1>{"joshument.dev"}</h1>
                    <nav>
                        <ul>
                            <li>
                                <Link<Route> to={Route::Home}>{"home"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::About}>{"about"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::Projects}>{"projects"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::RefSheet}>{"reference sheet"}</Link<Route>>
                            </li>
                            <li>
                                <Link<Route> to={Route::LegalStuff}>{"legal stuff"}</Link<Route>>
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
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}