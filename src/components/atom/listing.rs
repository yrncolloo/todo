use stylist::yew::styled_component;
use yew::{function_component, html, Html, Properties};
use crate::components::atom::checkbox::AACheckbox;

#[derive(Clone, PartialEq)]
struct TodoItems{
    id:usize,
    title:String

}

#[derive(Properties, PartialEq)]
struct ListProps{
    todoitems:Vec<TodoItems>

}

#[styled_component]
pub fn ListItems() -> Html{

    let items = vec![
        TodoItems{
            id:1,
            title:"Building todo application".to_string()
        },

        TodoItems{
            id:2,
            title:"How to cook".to_string()
        },
        TodoItems{
            id:3,
            title:"Reading rust".to_string()
        }
    ];


    html!{
        <>
        <Creatinglist todoitems={items}/> 
        </>

    }
}

#[function_component(Creatinglist)]
fn creatinglist(ListProps { todoitems }: &ListProps) -> Html{

    todoitems.iter().map(|list| html!{
        <p key={list.id}><AACheckbox> {format!("{}", list.title)}</AACheckbox>
            
 </p>
            }).collect()
    
}
