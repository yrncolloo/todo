use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Html,
    #[prop_or_default()]
    pub center:bool,
    pub level:AAheaderLevel
}


#[derive(PartialEq)]
pub enum AAheaderLevel {
    One,
    Two,
}



#[styled_component]
pub fn AAheading(props:&Props) -> Html{

    let mut styles:Vec<Style> = vec![];
    if props.center{
        styles.push(
            style!{
                text-align: center;

            }.unwrap()

            )
    };
        html!{
            if props.level == AAheaderLevel::One {
                <h1 class={classes!(styles)}> {props.children.clone()} </h1>
            }else {
                <h2 class={classes!(styles)}> {props.children.clone()} </h2>
            }
        }

}
