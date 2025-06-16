use crate::models::State;
use yew::{function_component, html, use_state, Callback, Html, HtmlResult};
use yewdux::prelude::*;

#[function_component(Showcase)]
pub fn secure() -> HtmlResult {
    Ok(html! {
        <div>
            <div class="btn">
            <a style="color:white;" href="https://github.com/corgijan/rust-dortmund-website/tree/development/src/components/showcase.rs" target="_blank">
                { "Take a Look at the component on GitHub " }
            </a>
            </div>
        <h1>{ "Showcase Store" }</h1>
        <ShowcaseStore />
        <h1>{ "Showcase useState" }</h1>
        <UseState />
        </div>
    })
}

#[function_component(ShowcaseStore)]
pub fn store() -> HtmlResult {
    let (state, dispatch) = use_store::<State>();
    let onclick = dispatch.reduce_mut_callback(|state| state.count += 1);

    Ok(html! {
        <div>
            <button class="btn"  {onclick}><h3>{ "Increment value" }</h3></button>
            <p>
                <b>{ "Current value: " }</b>
                {state.count}
            </p>
        </div>
    })
}
#[function_component(UseState)]
fn state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <button class="btn" {onclick}><h3>{ "Increment value" }</h3></button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
