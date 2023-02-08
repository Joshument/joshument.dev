use yew::{Properties, Children, Classes, Component, Context, Html, html};

#[derive(Properties, PartialEq)]
pub struct PageBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes
}

pub enum PageBodyMessages {
    /// At index (usize) add Html (Html)
    AddChild(usize, Html)
}

pub struct PageBody {
    children: Vec<Html>
}

impl Component for PageBody {
    type Message = PageBodyMessages;
    type Properties = PageBodyProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            children: ctx.props().children.clone().into_iter().collect(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::AddChild(index, html) => {
                self.children.insert(index, html);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <body id="pagebody" class={ctx.props().class.clone()}>
                { for Children::new(self.children.clone()).iter() }
            </body>
        }
    }
}