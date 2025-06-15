use gloo_net::http::Request;
use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;
use crate::app::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
        <div style="margin:30px">
            <Link<Route> classes={classes!("btn")} to={Route::UpcomingEventListRequest }>
                    { "View Events" }</Link<Route>>
        </div>
            <img class="logo" src="./assets/rust.webp" alt="rust do logo"  style="max-width:60vw;height:auto;border-radius:30px" />
            <br />
            <span class="subtitle">{ "this website is made in Dortmund with " }<span class="heart" /></span>
        </>
    }
}
