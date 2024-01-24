use yew::prelude::*;
mod components;

use components::atom::heading_one::BBheading;

#[function_component(App)]
pub fn app() -> Html{
    
    html!{
        <BBheading/>

    }
    
}
