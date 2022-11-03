use yew::prelude::*;
use crate::local_time;

pub fn page() -> Html {
    html! { 
        <div class="content">
            <h1>{"About"}</h1>
            <h2>{"Hi!"}</h2>
            <p>
                {"My name is Joshument, and I'm a programmer and wannabe mathematican that works on whatever they feel like. "}
                {"I tend to primarily write my code in Rust (as a matter of fact, this site was made in "}
                <a href="https://yew.rs/">{"Yew"}</a>{"!) and I don't focus on only one facet of programming."}
            </p>
            <p>
                {"In particular, I'm half-decent at writing low level code, and at one point would like to be able to write a kernel from scratch, "}
                {"but I also work on things like Discord bots and webites, as well as competitive programming and a lot of CLI applications."}
            </p>
            <p>
                {"I primarily use Linux, only using Windows/macOS if strictly necessary, but I tend to get along just fine with things like "}
                {"proton. "}<del>{"One day I'll set up a kvm for everything."}</del>{" If you care, my favourite distro is probably Gentoo Linux."} 
            </p>
            <p>
                {"Outside of programming, you'll probably find me playing a lot of indie games - some of my favourites being Celeste, OneShot, "}
                {"OMORI, Outer Wilds, Lobotomy Corporation, Library of Ruina, and probably a lot more I can't name off the top of my head. "}
                {"I also play a lot of rhythm games (my favourite being SDVX), and I use to play tetris competitively. "}
            </p>
            <p>
                {"If you need to contact me, you can find me on "}
                <a href="https://twitter.com/Joshument" target="_blank" rel="noopener noreferrer">{"Twitter"}</a>{". "}
                {"You might be able to DM me on "}
                <a href="https://discord.com/users/263785241292963860" target="_blank" rel="noopener noreferrer">{"Discord"}</a>
                {", but don't count on it."}
            </p>
            <p>
                <strong>{"00:00"}</strong>  {" for me is "}
                <strong>{local_time::get_local_time(1667361600, local_time::FormatOptions::HourMinute)}</strong>
                {" for you."}
            </p>
        </div>
    }
}