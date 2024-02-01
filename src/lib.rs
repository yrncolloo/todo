use stylist::{style, yew::styled_component};
use yew::prelude::*;
use components::{sidebar::Sidebar, center_part::Centerpart};

pub mod components;

#[styled_component(App)]
pub fn app() -> Html{
    
    let style = style!{
        border:0.1px solid #F4F4F4;
        background-color:#F4F4F4;
        height:600px;
        margin:20px;
        border-radius:25px;
    }.unwrap();

    html!{
        <div class={style}>
            <Sidebar/>
            <Centerpart/>

        </div>


    }
    
}
