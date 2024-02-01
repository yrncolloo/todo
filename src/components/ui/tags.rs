use stylist::{style, yew::styled_component};
use yew::{html, Html};
use crate::components::ui::button::But;

#[styled_component]
pub fn Tags() -> Html{

    let style = style!{
        .title{
            font-weight:bold;
        }
        .tags{
            display:inline-flex;
        }
        width:auto;
    }.unwrap();



    html!{
        <div class={style}>
            <p class={"title"}>{"TAGS"}</p>
        <div class={"tags"}>
            <But backgroundcolor={"red"}>{"#tag1"}</But>
            <But>{"#tag1"}</But>
            <But>{"#tag3"}</But>
        </div>
        </div>

    }
    
}
