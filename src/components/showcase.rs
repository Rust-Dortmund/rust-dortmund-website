use crate::models::State;
use yew::{function_component, html, HtmlResult};
use yewdux::prelude::*;

#[function_component(Showcase)]
pub fn secure() -> HtmlResult {
    let (state, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);
    Ok(html! {
        <div>
            <h6>{ "Count :" }
            {state.count}</h6>
            <button {onclick}>{"+1"}</button>
            <div>
            <div class="btn">
            <a style="color:white;" href="https://github.com/corgijan/rust-dortmund-website/tree/development/src/components/showcase.rs" target="_blank">
                { "Take a Look at the component on GitHub " }
            </a>
            </div>
            </div>
        </div>
    })
}
