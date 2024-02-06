use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Vec<String>,
}
#[styled_component]
pub fn Icons(props:&Props) -> Html{
   let style = style!{
       img{
                position: absolute;
                top: 50%;
                transform: translateY(-50%);
                pointer-events: none; /* Prevent the icon from blocking clicks on the input */
                left: 10px;
        }
   }.unwrap();
    html!{

        <div class={style}>
            {list_to_html(props.children.clone())}
        </div>
    }
}


fn list_to_html(list: Vec<String>) -> Vec<Html> {
    list.into_iter().map(|item | html!{<img src={item}/>}).collect()
        
}

