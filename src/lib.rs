use yew::prelude::*;
mod components;

use components::atom::heading_one::{AAheading, AAheaderLevel};
use crate::components::molecules::tab::MMTab;

#[function_component(App)]
pub fn app() -> Html{
    
    html!{
        <>
            <AAheading center={true} level={AAheaderLevel::One}> {"Todo Application"}</AAheading>
            <MMTab>{"Crafting/Ideas"}</MMTab>
            <MMTab>{"Doing"}</MMTab>
            <MMTab>{"Done"}</MMTab>
        </>


    }
    
}
