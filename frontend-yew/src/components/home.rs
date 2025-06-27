use crate::app::Route;
use crate::models::State;
use yew::{classes, function_component, html, Html};
use yew_router::prelude::*;
use yewdux::use_store;

#[function_component(Home)]
pub fn home() -> Html {
    let (state, _) = use_store::<State>();
    let mut message = None;
    if state.count > 0 {
        message = Some("Thank you for using the Showcase");
    }
    html! {
        <>
        {message}
        <div style="margin:30px">
            <Link<Route> classes={classes!("btn")} to={Route::UpcomingEventListRequest }>
                    { "View Events" }</Link<Route>>
        </div>
            <img class="logo" src="./assets/rust.webp" alt="rust do logo"  style="max-width:60vw;height:auto;border-radius:30px" />
            <br />
            <span class="subtitle">{ "this website is made in Dortmund with " }<span class="heart" />{" and "}<span class="crab" /></span>
        </>
    }
}
