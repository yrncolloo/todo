use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props{
    pub placeholder:String,
    pub icon:Icons
}

#[derive(PartialEq)]
pub enum Icons {
    Search,
    Add
}

#[styled_component]
pub fn Search(props:&Props) -> Html {
    let icons = match props.icon {
        Icons::Search => "fa fa-search".to_owned(),
        Icons::Add => "fa fa-plus".to_owned()
    };
    
    let style = style!{

               input[type=text] {
                   border: 1px solid #7A7A7A;
                    width: 90%;
                    border-radius: 10px;
                    padding: 10px;
                    background-color: #EFECEC;
                    }
        .icon-input-container{
            position:relative;
            display:inline;
            align-items:center;
        }
        .fa-search {
                position: absolute;
                top: 50%;
                right: 10px; /* Adjust the position of the icon */
                transform: translateY(-50%);
                pointer-events: none; /* Prevent the icon from blocking clicks on the input */
        }
    }.unwrap();
    
    html!{

        <div class={style}>
            <div class={"icon-input-container"}>
                <input type="text" placeholder={props.placeholder.clone()}/>
                <i class={icons}></i>
            </div>
        </div>
    }
}
