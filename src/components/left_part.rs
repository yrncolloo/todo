use stylist::{style, yew::styled_component};
use yew::{html, Html};

use crate::components::ui::description::Description;

#[styled_component]
pub fn Leftpart() -> Html{
    let style = style!{

        border:0.1px solid black;
        float:left;
        height:86%;
        width:20%;
        margin:2%;
        margin-left:0px;
        margin-right:0px;
        padding:20px;
        border-radius:25px;
        background-color:#EFECEC;
    }.unwrap();
    html!{

        <div class={style}>
            <h1>{"Tasks:"}</h1>
            <Description />
        </div>
    }
    
}
