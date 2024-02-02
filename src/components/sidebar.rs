use stylist::{style, yew::styled_component};
use yew::{html, Html};
use crate::components::ui::{title::Titles, title::TitleLevel};
use crate::components::ui::{search::Search, tasks::Tasks, tags::Tags, button::But};

#[styled_component(Sidebar)]
pub fn sidebar() -> Html{
    let style = style!{
        width:20%;
        height:86%;
        border-radius:25px;
        margin:2%;
        padding:20px;
        background-color:#EFECEC;
        overflow:auto;
        float:left;
        //Well for some reason I added this and scrollbar disappeared wth
        ::-webkit-scrollbar {
              width: 10px;
              height: 10px;
        }

    }.unwrap();

   
    let tasks:Vec<&str> = vec!["Upcoming", "Today", "Calendar", "Sticky Wall"];
    let list:Vec<&str> = vec!["Personal", "Work", "List 1"];
    html!{
        <div class={style}>
            <Titles level={TitleLevel::Two}>{"Menu"}</Titles>
            <Search placeholder={"Search"}/>
            <Tasks title={"TASKS"}>{tasks}</Tasks>
            <Tasks title={"LIST"}>{list}</Tasks>
            <But>{"Add List"}</But>
            <Tags/>
            <br/>
            <br/>
            <But>{"Settings"}</But>
            <But>{"Sign Out"}</But>
        </div>

    }
}
