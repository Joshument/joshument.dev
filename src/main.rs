mod gif_img;

use yew::prelude::*;
use gif_img::GifImg;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <header>
                <h1>{"joshument.dev"}</h1>
                <nav>
                    <ul>
                        <li><a href="/">{"home"}</a></li>
                        <li><a href="about">{"about"}</a></li>
                        <li><a href="projects">{"projects"}</a></li>
                        <li><a href="refsheet">{"reference sheet"}</a></li>
                    </ul>
                </nav>
            </header>

            <div class="content" role="list">
                <h2>{"Welcome to my website!"}</h2>
                <p>{"I use this website to host my project portfolio as well as other information about me."}</p>
                <ul class="homepagehyperlinklist">
                    <li>{"If you'd like to know more about me, you can check out my "}<a href="about">{"about page"}</a></li>
                    <li>{"If you'd like to make art involving me (thank you!) you can check out my "}<a href="refsheet">{"reference page"}</a></li>
                    <li>{"If you'd like to know more about my projects, you can check out my "}<a href="projects">{"projects page"}</a></li>
                </ul>
                <p>
                    {
                        "This is my main subdomain, and thus is about me and not my projects. \
                        If you are here for my discord bot, you can probably find what you need at "
                    }
                    <a href="https://jolt.joshument.dev">{"jolt.joshument.dev"}</a>
                </p>

                <h2>{"Why make a website?"}</h2>
                <p>{"
                    Making a website allows me to have a lot more freedom in how I set things up (compared to, say, a carrd). \
                    It's also a good opportunity for me to practice web development, and I get a hell of a lot more space than \
                    a regular bio.
                "}</p>
            </div>

            <footer>
                {"This website is open source! Feel free to check out the "}
                <a href="https://github.com/Joshument/joshument.dev">{"source code"}</a>
            </footer>

            <div class="lily">
                <GifImg gif="img/movinglily.gif" img="img/lily.png" alt="lily" gifalt="lily" width="48" height="48"/>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}