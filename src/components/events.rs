use crate::app::Route;
use crate::events::events;
use crate::models::Event;
use crate::models::_News::title;
use gloo_net::http::Request;
use std::sync::Arc;
use yew::suspense::use_future;
use yew::{classes, function_component, html, use_effect_with, use_state, Html, HtmlResult, Properties, Suspense};
use yew_router::components::Link;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub event: Event,
}

#[function_component(SingleEvent)]
pub fn secure(prop: &Props) -> Html {
    let event = &prop.event;
    let is_upcoming_event = prop.event.date > chrono::Local::now().naive_local().date();
    let date_text = if is_upcoming_event {
        format!("Upcoming Event: {}", prop.event.date)
    } else {
        format!("Past Event: {}", prop.event.date)
    };
    html! {
        <div>
            <div class="event" key={event.id}>
                <h2>{ &event.title }</h2>
                <p>{ &event.description }</p>
                <p>{ &date_text }</p>
        {            event.talks.iter().map(|talk| {
                    html! {
                        <div class="talk">
                            <h5>{ &talk.title }</h5>
                        </div>
                }
                }
        ).collect::<Html>()
            }
                <p>{ format!("Location: {}", event.location) }</p>
            <img class="event-image" src={event.image_url.clone()} />
            </div>
        </div>
    }
}

#[function_component(SingleEventSmall)]
pub fn secure(prop: &Props) -> Html {
    let is_upcoming_event = prop.event.date > chrono::Local::now().naive_local().date();
    let date_text = if is_upcoming_event {
        format!("Upcoming Event: {}", prop.event.date)
    } else {
        format!("Past Event: {}", prop.event.date)
    };
    html! {
        <div>
            <h1>{ &prop.event.title }</h1>
            <h3>{ date_text }</h3>
            <p>{ &prop.event.description }</p>
        <p>
            {
                prop.event.talks.iter().map(|talk| {
                    html! {
                    <div>
                            { format!(" {} by  {}",talk.title,talk.speaker) }
                    </div>
                    }
                }).collect::<Html>()
            }
        </p>
        <p>
            <Link<Route> classes={classes!("btn")} to={Route::EventsRequest { id: prop.event.id as u16}}>
                    { "View Event" }</Link<Route>>
            </p>
        </div>
    }
}

#[function_component(Upcoming)]
pub fn secure() -> Html {
    let events = events();
    events
        .into_iter()
        .map(|event| {
            html! {
                    <>
                    <SingleEventSmall  event={event.clone()} />
                    <hr />
                    </>
            }
        })
        .collect::<Html>()
}

#[function_component(RequestTest)]
pub fn secure() -> HtmlResult {
    const URL: &str = "https://en.wikipedia.org/w/api.php?\
                   action=query&origin=*&format=json&generator=search&\
                   gsrnamespace=0&gsrlimit=5&gsrsearch='New_England_Patriots'";
    let res = use_future(|| async { Request::get(URL).send().await?.text().await })?;

    let result_html = match *res {
        Ok(ref res) => html! { res },
        Err(ref failure) => html! {
                { format!("Error fetching data: {}", failure) }
        },
    };
    return Ok(html! {
        <div>
            <h6>{ "Test" }
                { result_html }
            </h6>
        </div>
    });
}
