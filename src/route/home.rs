use yew::prelude::*;

pub fn page() -> Html {
    html! { 
        <div class="content" role="list">
            <h2>{"Welcome to my website!"}</h2>
            <p>{"I use this website to host my project portfolio as well as other information about me."}</p>
            <ul class="homepagehyperlinklist">
                <li>{"If you'd like to know more about me, you can check out my "}<a href="about">{"about page"}</a></li>
                <li>{"If you'd like to know more about my projects, you can check out my "}<a href="projects">{"projects page"}</a></li>
                <li>{"If you'd like to make art involving me (thank you!) you can check out my "}<a href="refsheet">{"reference page"}</a></li>
            </ul>
            <p>
                {
                    "This is my main subdomain, and thus is about me and not my projects. \
                    If you are here for my Discord bot, you can probably find what you need at "
                }
                <a href="https://jolt.joshument.dev">{"jolt.joshument.dev"}</a>
            </p>
    
            <h2>{"Why make a website?"}</h2>
            <p>{"
                Making a website allows me to have a lot more freedom in how I set things up (compared to, say, a Carrd). \
                It's also a good opportunity for me to practice web development, and I get a hell of a lot more space than \
                a regular bio.
            "}</p>
        </div>
    }
}