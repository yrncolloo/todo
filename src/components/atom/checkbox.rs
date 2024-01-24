use stylist::{style, yew::styled_component};
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children:Html
}

#[styled_component(AACheckbox)]
pub fn checkbox(props:&Props) -> Html{

    let styles = style!{
                input[type=checkbox]
                {
                  /* Double-sized Checkboxes */
                  -ms-transform: scale(2); /* IE */
                  -moz-transform: scale(2); /* FF */
                  -webkit-transform: scale(2); /* Safari and Chrome */
                  -o-transform: scale(2); /* Opera */
                  transform: scale(2);
                  padding: 10px;
                }
                
                /* Might want to wrap a span around your checkbox text */
                .Checkbox
                {
                  /* Checkbox text */
                  font-size: 110%;
                  display: inline;
                  margin-left:10px;
                }
    }.unwrap();


    html!{

        <>
        <label class={styles.clone()}>
          <input type="checkbox"/>
          <span class="Checkbox">{props.children.clone()}</span>
        </label>
        </>

    }

    
}
