use yew::{function_component, html, Html};
use gloo_net::http::Request;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1>{ "Hello World!" }</h1>
            <img class="logo" src="./assets/rust.webp" alt="rust do logo"  style="max-width:60vw;height:auto;border-radius:30px" />
            <br />
            <span class="subtitle">{ "this website is made in Dortmund with " }<i class="heart" /></span>
        </>
    }
}
