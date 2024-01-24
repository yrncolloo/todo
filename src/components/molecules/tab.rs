use stylist::{style, yew::styled_component};
use yew::{html, Html};

use crate::components::atom::listing::ListItems;

#[styled_component(MMTab)]
pub fn tab() -> Html{

    let stylesheet = style!{
        background-color: blue;
        margin-right: 10px;
        margin-left:10px;
        margin-top:0px;
        margin-bottom:10px;
        border-radius:5px;
        padding: 20px;
        float: left;
        width:260px;
    }.unwrap();


    html!{
        <div class={stylesheet}>
        <ListItems/>
        </div>

    }
    
}
