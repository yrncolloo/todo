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
            flex-wrap:wrap;
        }
        width:auto;
    }.unwrap();

    let tag:Vec<&str> = vec!["#Personal", "#Work", "#tag1", "#tag2", "#tag3", "#tag"];


    html!{
        <div class={style}>
            <p class={"title"}>{"TAGS"}</p>
        <div class={"tags"}>
        {list_to_html(tag)}
        </div>
        </div>

    }
    
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.into_iter().map(|item | html!{
        <But>{item}</But>
    }).collect()
}
