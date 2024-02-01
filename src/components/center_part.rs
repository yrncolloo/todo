use stylist::{style, yew::styled_component};
use yew::{html, Html};

use crate::components::ui::{title::Titles, title::TitleLevel};

#[styled_component]
pub fn Centerpart() -> Html{
    
    let style = style!{

        border:0.1px solid black;
        float:left;
        width:45%;
        height:86%;
        padding:20px;
        border-radius:25px;
        margin:2%;
        margin-left:0px;
    }.unwrap();
    html!{
        <div class={style}>
            <Titles level={TitleLevel::One}>{"Today"}</Titles>
        </div>

    }
}
