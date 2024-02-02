use stylist::{style, yew::styled_component};
use yew::{html, Html};

#[styled_component]
pub fn Description() -> Html {

    let style = style!{
        .textarea{
            background-color:#EFECEC;
            width:94%;
            border-radius:15px;
            padding:10px;
        }

    }.unwrap();
    html!{

        <div class={style}>
            <textarea rows="5" cols="10" class={"textarea"} placeholder={"Description"}></textarea>
            
        </div>
    }
    
}
