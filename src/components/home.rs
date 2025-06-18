use crate::app::Route;
use crate::models::State;
use gloo_net::http::Request;
use yew::suspense::use_future;
use yew::{classes, function_component, html, Html, HtmlResult};
use yew_router::prelude::*;
use yewdux::use_store;

#[function_component(Authcheck)]
pub fn home() -> HtmlResult {
    const URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";
    let res = use_future(|| async move {
        Request::get(URL)
            .header("Authorization", &format!("Bearer {}", ""))
            .send()
            .await?
            .text()
            .await
    })?;

    let result_html = match *res {
        Ok(ref res) => html! {
            res
        },
        Err(ref failure) => html! {
                { format!("Error fetching data: {}", failure) }
        },
    };
    Ok(result_html)
}

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
