use stylist::{style, yew::styled_component};
use yew::{html, Html};

#[styled_component]
pub fn Search() -> Html {
    
    let style = style!{

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
            <input type="text" placeholder="Search.."/>
        </div>
    }
}
