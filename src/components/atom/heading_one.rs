use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:String

}



#[styled_component]
pub fn BBheading(props:&Props) -> Html{

    let center = style!{
        text-align: center;
        color: red;
    }.unwrap();


    html!{
            <h1 class={center}> {props.children.clone()} </h1>

    }
}
