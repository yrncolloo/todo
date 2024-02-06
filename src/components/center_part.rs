use stylist::{style, yew::styled_component};
use yew::{html, Html};

use crate::components::ui::{title::Titles, title::TitleLevel, search::Search, search::Icons};
use crate::components::ui::{center_box::Box};

#[styled_component]
pub fn Centerpart() -> Html{
    
    let style = style!{

        float:left;
        width:42vw;
        height:81vh;
        padding:1%;
        border-radius:25px;
        margin:1%;
        margin-left:0px;
        margin-right:0px;
    }.unwrap();
    html!{
        <div class={style}>
            <Titles level={TitleLevel::One}>{"Today"}</Titles>
            <Search icon={Icons::Add} placeholder={"Add new task"}/>
            <Box/>

        </div>

    }
}
