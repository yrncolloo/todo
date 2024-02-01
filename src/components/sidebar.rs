use stylist::{style, yew::styled_component};
use yew::{html, Html};
use crate::components::ui::{title::Titles, title::TitleLevel};
use crate::components::ui::search::Search;

#[styled_component(Sidebar)]
pub fn sidebar() -> Html{
    let style = style!{
        border:1px solid black;
        width:20%;
        height:86%;
        border-radius:25px;
        margin:20px;
        padding:20px;
        background-color:#EFECEC;
    }.unwrap();
   
    html!{
        <div class={style}>
            <Titles level={TitleLevel::Two}>{"Menu"}</Titles>
            <Search/>
            <Titles level={TitleLevel::Five}>{"TASKS"}</Titles>
        </div>

    }
}
