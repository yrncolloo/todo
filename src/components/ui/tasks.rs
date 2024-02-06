use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};
use crate::components::ui::button::But;
use crate::components::ui::icons::Icons;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Vec<&'static str>,
    pub title:String,
    pub icon:Image
}

#[derive(PartialEq)]
pub enum Image {
    Task,
    List,
}


#[styled_component]
pub fn Tasks(props:&Props) -> Html{

    let icon_to_render = match props.icon {
        Image::Task => vec![
            "images/angles-right-solid.svg".to_string(),
            "images/bars-solid.svg".to_string(),
            "images/calendar-days-solid.svg".to_string(),
            "images/book-solid.svg".to_string(),
        ],
        Image::List => vec![
            "images/square-solid-red.svg".to_string(),
            "images/square-solid-blue.svg".to_string(),
            "images/square-solid-yellow.svg".to_string(),
        ]

    };
    
    let style = style!{

        .title{
            font-weight:bold;
        }

    }.unwrap();


    html!{
        <div class={style}>

            <div class={"title"}>
                <p>{props.title.clone()}</p>
            </div>
               {list_to_html(props.children.clone(), icon_to_render.clone())}

        </div>

    }
}

fn list_to_html(list: Vec<&str>, taskss:Vec<String>) -> Vec<Html> {
    // list.into_iter().map(|item | html!{<But icon={Some(html!{<Icons>{taskss.clone()}</Icons>})}>{item}</But>}).collect()
    list.into_iter().zip(taskss).map(|(item, icon)| html!{<But icon={Some(html!{<Icons children={vec![icon]}/>})}>{item}</But>}).collect()
}

