mod components;
mod router;
mod pages;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use components::typing_anim::{TypingAnim, TypingAnimState};
use gloo::console;
use yew;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew_router::prelude::*;

use components::toggle_dark::*;

#[derive(Properties, PartialEq)]
struct RootComponentProperties {
    #[prop_or_default]
    class: Classes,
    #[prop_or_default]
    id: String,
    #[prop_or_default]
    children: Children
}

enum RootComponentMessage {
    AddChild(usize, Html),
    RemoveChild(usize)
}

struct RootComponent {
    children: Vec<Html>
}

impl Component for RootComponent {
    type Properties = RootComponentProperties;
    type Message = RootComponentMessage;

    fn create(ctx: &Context<Self>) -> Self {
        let on_finish_propagate = ctx.link().callback(|_| Self::Message::AddChild(1, html! {<p>{"test"}</p>}));
        let mut children = vec![
            html! {
                <TypingAnim 
                    class={classes!(
                        "transition-all", "duration-700", "mb-0", "mt-52",
                        "text-fuchsia-500", r"after:content-['\_']", "m-auto", "text-3xl"
                    )} 
                    text={"joshument.dev"} 
                    interval={std::time::Duration::from_millis(100)}
                    on_finish={Callback::from(|component: Rc<TypingAnimState>| {
                        *component.class.as_ref().unwrap().borrow_mut() = component.class.as_ref().unwrap().borrow().clone()
                            .into_iter()
                            .filter(|x| {
                                x.deref() != "mt-52"
                            })
                            .collect::<Classes>();
                    })}
                    {on_finish_propagate}
                />
            }
        ];
        children.push(ctx.props().children.clone().into_iter().collect());

        Self {
            children: children
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::AddChild(index, element) => {
                self.children.insert(index, element);
                true
            },
            Self::Message::RemoveChild(index) => {
                self.children.remove(index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div id={props.id.clone()} class={props.class.clone()}>
                {for self.children.iter().cloned()}
            </div>
        }
    }
}

#[function_component]
fn App() -> Html {
    html!(
        <div id={"root"} class={classes!("dark")}>
            <RootComponent class={classes!(
                "font-mono", "bg-slate-200", "dark:bg-slate-800", "w-full", "h-screen",
                "flex", "items-center", "justify-between", "flex-col", "h-screen"
            )}>
            </RootComponent>
        </div>
        
        /*
        <div id={"root"} class={classes!("dark")}>
            <body class={classes!("font-mono", "bg-slate-200", "dark:bg-slate-800", "w-full", "h-screen")}>
                <BrowserRouter>
                    <Switch<router::Route> render={router::switch}/>
                </BrowserRouter>
            </body>
        </div>
        */
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}