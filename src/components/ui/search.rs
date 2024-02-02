use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub placeholder:String,
}

#[styled_component]
pub fn Search(props:&Props) -> Html {
    
    let style = style!{

               input[type=text] {
                   border: 1px solid #7A7A7A;
                    width: 90%;
                    border-radius: 10px;
                    padding: 10px;
                    background-color: #EFECEC;
                    }
                    background: url("/home/yrncollo/Pictures/wallpaper1.jpg");
    }.unwrap();
    
    html!{

        <div class={style}>
            <input type="text" placeholder={props.placeholder.clone()}/>
        </div>
    }
}
