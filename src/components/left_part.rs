use stylist::{style, yew::styled_component};
use yew::{html, Html};

use crate::components::ui::{title::Titles, title::TitleLevel, tags::Tags};

use crate::components::ui::description::Description;
use crate::components::ui::search::Search;

#[styled_component]
pub fn Leftpart() -> Html{
    let style = style!{

        float:left;
        height:81vh;
        width:22vw;
        margin:1%;
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
        overflow:auto;
        //Well for some reason I added this and scrollbar disappeared wth
        ::-webkit-scrollbar {
              width: 10px;
              height: 10px;
        }

    }.unwrap();
    html!{

        <div class={style}>
            <Titles level={TitleLevel::Two}>{"Task:"}</Titles>
            <p class={"task"}>{"Renew driver licence"}</p>
            <Description />
            <Tags/>
            <Addtag/>
        </div>
    }
    
}

#[styled_component]
fn Addtag() -> Html{
    let style = style!{
        #toggleCheckbox{
            display:none;
        }
        #toggleButton{
            display:inline-block;
            padding:10px;
            background-color:#EFECEC;
            border-radius:10px;
            border: 1px dotted #7A7A7A;
        }
        #toggleButton:hover{
            background-color:#CECECE;
            color:black;
            cursor:pointer;
            border: 1px solid #7A7A7A;
        }
        #textBox{
            display:none;
        }
        #toggleCheckbox:checked ~ #textBox{
            display:block;
        }
               input[type=text] {
                   border: 1px solid #7A7A7A;
                    width: 90%;
                    border-radius: 10px;
                    padding: 10px;
                    background-color: #EFECEC;
                    }
    }.unwrap();

    html!{
        <div class={style}>
            <input type={"checkbox"} id="toggleCheckbox" />
            <label for = "toggleCheckbox" id="toggleButton">{"Add Tag"}</label>
            <input type={"text"} id="textBox" placeholder={"Add your tag here"} />
        </div>
    }
}
