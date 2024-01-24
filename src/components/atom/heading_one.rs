use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component]
pub fn BBheading() -> Html{

    let center = style!{
        text-align: center;
        color: red;
    }.unwrap();


    html!{
            <h1 class={center}> {"Todo Application"}</h1>

    }
}
