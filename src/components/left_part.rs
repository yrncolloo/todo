use stylist::{style, yew::styled_component};
use yew::{html, Html};

use crate::components::ui::{title::Titles, title::TitleLevel, tags::Tags};
use crate::components::ui::description::Description;

#[styled_component]
pub fn Leftpart() -> Html{
    let style = style!{

        float:left;
        height:86%;
        width:20%;
        margin:2%;
        margin-left:0px;
        margin-right:0px;
        padding:20px;
        border-radius:25px;
        background-color:#EFECEC;
        .task{
            border: 1px solid #7A7A7A;
            padding: 10px;
            border-radius: 10px;
        }
    }.unwrap();
    html!{

        <div class={style}>
            <Titles level={TitleLevel::Two}>{"Task:"}</Titles>
            <p class={"task"}>{"Renew driver licence"}</p>
            <Description />
            <Tags/>
        </div>
    }
    
}
