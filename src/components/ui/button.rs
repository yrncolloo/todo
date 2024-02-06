use stylist::{style};
use yew::{function_component, html, Html, Properties};
use crate::components::ui::icons::Icons;


#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Html,
    #[prop_or_default()] // I Dont know how to implement this yet
    pub backgroundcolor:String,
    #[prop_or_default()]
    pub icon:Option<Html>,
    #[prop_or_default()]
    pub icon_image:Vec<String>
}

#[function_component]
pub fn But(props:&Props) -> Html{

    let style = style!{
        .button {

            border:none;
            width:100%;
            border-radius:10px;
            padding:10px;
            padding-left:20%;
            text-align:left;
            font-size:15px;
            color:#616161;
            background-color:#EFECEC;
            margin:2px;
        }
        .button:hover{
            background-color:#CECECE;
            color:black;
            cursor:pointer;
        }
        position:relative;

    }.unwrap();

    html!{

        <div class={style}>
        <button class={"button"}>{props.children.clone()}</button>
        <Icons > {props.icon_image.clone()}</Icons>
        {props.icon.clone()}
        </div>
    }
}
