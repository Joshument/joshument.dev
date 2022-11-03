use yew::prelude::*;

pub fn page() -> Html {
    html! { 
        <div class="refsheet">
            <div class="content">
                <h1>{"Reference Sheet"}</h1>
                <h2>{"Thanks for your interest!"}</h2>
                <p>
                    {"Regardless of why you're here, I appreciate your interest in knowing my design. If you choose to add art, I may add it here "}
                    {" with your permission to both promote your work and show off more pieces of my design."}
                </p>
                <p>
                    {"Some things to note: "}
                </p>
                <ul class="fira-arrow-unordered-list">
                    <li>{"This characer is of myself - any references to \"me\" also apply to this character."}</li>
                    <li>{"I'm 5'3, so not super tall."}</li>
                    <li>
                        {"If you need a reference to how I should act (expressions / poses), I would reference my "}
                        <a href="https://twitter.com/Joshument">{"Twitter"}</a>
                        {"."}
                        <ul>
                            <li>{"If you care about personality types, I am an INTP 5w4 sp/sx, but taking these as gosepl is a bad idea."}</li>
                        </ul>
                    </li>
                    <li>{"Feel free to change up "}<em>{"anything"}</em>{" about the design. I like seeing how other people interpret me!!"}</li>
                </ul>
            </div>

            <ul class="refsheet-images">
                <div class="row-1">
                    <img 
                        src="img/FullBodyFront.png" 
                        alt="\
                            Reference image for the front of my design. \
                            I am wearing a purple top hat with a blue stripe at the base, \
                            a black baggy hoodie with 2 straight purple streaks on each arm near the top, \
                            and a pair of black thigh-highs with the same purple streak. \
                        "
                    />
                    <div class="flexbox-column">
                        <img 
                            src="img/PeaceSign.png" 
                            alt="\
                                Extra image for the design. \
                                I am holding up the peace sign / v-sign with a more mellow expression. \
                            "
                        />
                        <img 
                            src="img/FullBodyBack.png" 
                            alt="\
                                Reference image for the back of my design. \
                                The back of the hair is shown to be slightly below the armpits. \
                            "
                        />
                    </div>
                </div>
            </ul>
        </div>
    }
}