use crate::events::events;
use crate::models::Event;
use crate::nav::Route;
use gloo_net::http::Request;
use yew::suspense::use_future;
use yew::{Html, HtmlResult, Properties, classes, function_component, html};
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
            <img class="event-image" src={event.image_url.clone()} />
            <div class="event" key={event.id}>
                <h2>{ &event.title }</h2>
                <p>{ &event.description }</p>
                <p>{ &date_text }</p>
        {            event.talks.iter().map(|talk| {
                    html! {
                        <div class="talk">
                            <h3>{ &talk.title }</h3>
                            <p>{" by "} { &talk.speaker }</p>

                            {if let Some(video_url) = &talk.video_url {
                                html! { <> {"- "} <a href={video_url.clone()} target="_blank">{ "Watch Video" }</a>{" -"} </>}
                                } else {
                                html! { <span>{ "" }</span> }
                            }
                            }
                            {if let Some(slides_url) = &talk.slides_url {
                                html! {<> {"- "} <a href={slides_url.clone()} target="_blank">{ "Download Slides" }</a> {" -"}</> }
                                } else {
                                html! { <span>{ "" }</span> }
                            }
                            }

                        </div>
                }
                }
        ).collect::<Html>()
            }
                <p>{ format!("Location: {}", event.location) }</p>
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

        <>
            <h2>{ &prop.event.title }</h2>
            <h3>{ date_text }</h3>
            <p>{ &prop.event.description }</p>
        if prop.event.talks.is_empty() {
            <p>{ "No information on talks." }</p>
        } else {
            <ul>
                {
                    prop.event.talks.iter().map(|talk| {
                        html! {
                        <li>
                          <b>{talk.title}</b><br />{" by "}<u>{talk.speaker}</u>
                        </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
        }
            <div>
                <Link<Route> classes={classes!("btn")} to={Route::EventsRequest { id: prop.event.id as u16}}>
                    { "View Event" }</Link<Route>>
            </div>
        </>
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
    const URL: &str = "https://en.wikipedia.org/w/api.php?action=query&origin=*&format=json&generator=search&gsrnamespace=0&gsrlimit=5&gsrsearch='New_England_Patriots'";
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
