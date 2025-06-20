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
            .header("Authorization", &format!("Bearer {}", "ya29.a0AW4XtxgWMd3Pzjm0pAkDnhYYWO6PGeJFTRMhY1g_IgR5HTIYnE_OquscnxINqVk5jZIlMGvATQl8DjBtD4sPoVOnDzJuDo4ztVM0kThmXA_EePi9aQuaqRudq_TXYqGJ0frz8TbfbQ1YlpiV6YqR_V6Y4FNaFxtiETKrKiN0VQaCgYKAUUSARUSFQHGX2MicsPay1kd6o83qoBeYJ2B7Q0177"))
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
