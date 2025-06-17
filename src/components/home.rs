use crate::app::Route;
use crate::models::{Event, State};
use gloo_net::http::Request;
use yew::suspense::use_future;
use yew::{classes, function_component, html, use_context, Callback, HtmlResult};
use yew_oauth2::context::OAuth2Context;
use yew_router::prelude::*;
use yewdux::use_store;

#[function_component(Home)]
pub fn home() -> HtmlResult {
    let (state, _) = use_store::<State>();
    let location = use_location().expect("location should be available");
    let query = location.query_str();

    // Parse query string into key-value pairs
    let params: Vec<(String, String)> = form_urlencoded::parse(query.as_bytes())
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    gloo::console::log!(format!("Query parameters: {:?}", params));
    let code = params
        .iter()
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.clone());
    let code2 = code.clone();
    gloo::console::log!(format!(
        "Query code: {:?}",
        code2.clone().unwrap_or("No code found".to_string())
    ));
    let state = state.clone();
    const URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";
    let code3 = code.clone();

    let res = use_future(|| async move {
        if let Some(token) = code3 {
            gloo::console::log!(format!("Token found: {}", token));
            let res = Request::get(URL)
                .header("Authorization", &format!("Bearer {}", token))
                .send()
                .await
                .unwrap()
                .text()
                .await;
            gloo::console::log!(format!("Response: {:?}", res));
        } else {
            gloo::console::log!("No token found in query parameters");
        }
    })?;
    let (state, dispatch) = use_store::<State>();
    if let Some(code) = code {
        gloo::console::log!(format!("Code found: {}", code));
        let _: Callback<Event> =
            dispatch.reduce_mut_callback(move |state| state.user = Some(code.clone()));
    } else {
        gloo::console::log!("No code found in query parameters");
    }

    let auth = use_context::<OAuth2Context>();

    let mut message = None;
    if state.count > 0 {
        message = Some("Thank you for using the Showcase");
    }
    Ok(html! {
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
    })
}
