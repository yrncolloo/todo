use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};
use crate::components::ui::button::But;

#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Vec<&'static str>,
    pub title:String,
}

#[styled_component]
pub fn Tasks(props:&Props) -> Html{
    
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
               {list_to_html(props.children.clone())}

        </div>

    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.into_iter().map(|item | html!{<But>{item}</But>}).collect()
}

