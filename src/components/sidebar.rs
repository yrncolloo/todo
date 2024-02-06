use stylist::{style, yew::styled_component};
use yew::{html, Html};
use crate::components::ui::{title::Titles, title::TitleLevel, search::Icons};
use crate::components::ui::{search::Search, tasks::{Tasks, Image }, tags::Tags, button::But};

#[styled_component(Sidebar)]
pub fn sidebar() -> Html{
    let style = style!{
        width:20vw;
        height:81vh;
        border-radius:25px;
        margin:1%;
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
    let add_list_icon:Vec<String> = vec!["images/plus-solid.svg".to_string()];
    let settings_icon:Vec<String> = vec!["images/sliders-solid.svg".to_string()];
    let sign_out_icon:Vec<String> = vec!["images/arrow-right-from-bracket-solid.svg".to_string()];
    html!{
        <div class={style}>
            <Titles level={TitleLevel::Two}>{"Menu"}</Titles>
            <Search icon={Icons::Search}placeholder={"Search"}/>
            <Tasks title={"TASKS"} icon={Image::Task}>{tasks}</Tasks>
            <Tasks title={"LIST"} icon={Image::List}>{list}</Tasks>
            <But icon_image={add_list_icon}>{"Add List"}</But>
            <Tags/>
            <br/>
            <br/>
            <But icon_image={settings_icon}>{"Settings"}</But>
            <But icon_image={sign_out_icon}>{"Sign Out"}</But>
        </div>

    }
}
