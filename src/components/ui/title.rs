use yew::{function_component, html, virtual_dom::VNode, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Html,
    pub level:TitleLevel,
}

#[derive(PartialEq)]
pub enum TitleLevel {
    One,
    Two,
    Three,
    Four,
    Five,
}


impl TitleLevel {
    pub fn render(&self, children:VNode) -> Html {
        match self {
            Self::One => html! {<h1>{children}</h1>},
            Self::Two => html! {<h2>{children}</h2>},
            Self::Three => html! {<h3>{children}</h3>},
            Self::Four => html! {<h4>{children}</h4>},
            Self::Five => html! {<h5>{children}</h5>},
        }
        }
}

#[function_component]
pub fn Titles(props:&Props) -> Html{
    
    props.level.render(props.children.clone())
}
