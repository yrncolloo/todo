use stylist::{style, yew::styled_component};
use yew::{html, Html};

#[styled_component]
pub fn Box() -> Html{
    let lists = vec!["Research content Idea", "Create database for guest authors", "Task 3"];
    let style = style!{
        padding: 1rem;
        font-size: 19px;
        font-weight: medium;

                .checkbox{
                    margin-right: 1rem;
                }

    }.unwrap();
    html!{

        <div class={style}>
            {list_to_html(lists)}
        </div>
    }
    
}
fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.into_iter().map(|item | html!{
        <>
            <input type="checkbox" class={"checkbox"} name={"inp"}/>
            <label for={"inp"}>{item}</label>
            <hr/>
        </>
    }).collect()
}

