use std::fmt::format;

use stylist::{style};
use yew::{function_component, html, AttrValue, Html, Properties};
#[derive(Properties, PartialEq)]
pub struct Props{
    pub children:Html,
    #[prop_or_default()] // I Dont know how to implement this yet
    pub backgroundcolor:String,
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

    }.unwrap();
    
    html!{

        <div class={style}>
        <button class={"button"}>{props.children.clone()}</button>
        </div>
    }
}
