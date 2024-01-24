use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Html,
    #[prop_or_default()]
    pub center:bool,
    pub level:BBheaderLevel
}


#[derive(PartialEq)]
pub enum BBheaderLevel {
    One,
    Two,
}



#[styled_component]
pub fn BBheading(props:&Props) -> Html{

    let mut styles:Vec<Style> = vec![];
    if props.center{
        styles.push(
            style!{
                text-align: center;
                color: red;

            }.unwrap()

            )
    };
        html!{
            if props.level == BBheaderLevel::One {
                <h1 class={classes!(styles)}> {props.children.clone()} </h1>
            }else {
                <h2 class={classes!(styles)}> {props.children.clone()} </h2>
            }
        }

}
