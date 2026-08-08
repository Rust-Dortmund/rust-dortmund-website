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

fn event_status(event: &Event) -> (bool, String) {
    let is_upcoming = event.date > chrono::Local::now().naive_local().date();
    let label = if is_upcoming {
        format!("Upcoming · {}", event.date)
    } else {
        format!("Past · {}", event.date)
    };
    (is_upcoming, label)
}

#[function_component(SingleEvent)]
pub fn secure(prop: &Props) -> Html {
    let event = &prop.event;
    let (is_upcoming, status_label) = event_status(event);

    html! {
        <div class="event-detail" key={event.id}>
            if !event.image_url.is_empty() {
                <div class="event-detail-image">
                    <img src={event.image_url.clone()} alt={event.title.clone()} />
                </div>
            }
            <div class="event-detail-body">
                <span class={classes!("event-badge", if is_upcoming { "upcoming" } else { "past" })}>
                    { status_label }
                </span>
                <h1>{ &event.title }</h1>
                if !event.description.is_empty() {
                    <p class="event-description">{ &event.description }</p>
                }
                if !event.location.is_empty() {
                    <p class="event-location">{ "📍 " }{ &event.location }</p>
                }
                if !event.talks.is_empty() {
                    <div class="talks-list">
                        { event.talks.iter().map(|talk| {
                            html! {
                                <div class="talk-card">
                                    <h3>{ &talk.title }</h3>
                                    <p class="talk-speaker">{ "by " }{ &talk.speaker }</p>
                                    if !talk.description.is_empty() {
                                        <p class="talk-description">{ &talk.description }</p>
                                    }
                                    if talk.video_url.is_some() || talk.slides_url.is_some() {
                                        <div class="talk-links">
                                            if let Some(video_url) = &talk.video_url {
                                                <a class="talk-link" href={video_url.clone()} target="_blank" rel="noopener noreferrer">
                                                    { "▶ Watch Video" }
                                                </a>
                                            }
                                            if let Some(slides_url) = &talk.slides_url {
                                                <a class="talk-link" href={slides_url.clone()} target="_blank" rel="noopener noreferrer">
                                                    { "📄 Slides" }
                                                </a>
                                            }
                                        </div>
                                    }
                                </div>
                            }
                        }).collect::<Html>() }
                    </div>
                }
                <div class="event-detail-back">
                    <Link<Route> classes={classes!("btn")} to={Route::UpcomingEventListRequest}>
                        { "← All Events" }
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}

#[function_component(SingleEventSmall)]
pub fn secure(prop: &Props) -> Html {
    let event = &prop.event;
    let (is_upcoming, status_label) = event_status(event);
    let speakers = event
        .talks
        .iter()
        .map(|talk| talk.speaker)
        .collect::<Vec<_>>()
        .join(", ");

    html! {
        <div class="event-card">
            if !event.image_url.is_empty() {
                <div class="event-card-image">
                    <img src={event.image_url.clone()} alt={event.title.clone()} loading="lazy" />
                </div>
            }
            <div class="event-card-body">
                <span class={classes!("event-badge", if is_upcoming { "upcoming" } else { "past" })}>
                    { status_label }
                </span>
                <h2>{ &event.title }</h2>
                if !event.description.is_empty() {
                    <p class="event-card-description">{ &event.description }</p>
                }
                if event.talks.is_empty() {
                    <p class="event-card-meta">{ "No talk details available yet." }</p>
                } else {
                    <p class="event-card-meta">
                        { format!("{} talk{} · {}", event.talks.len(), if event.talks.len() == 1 { "" } else { "s" }, speakers) }
                    </p>
                }
                <Link<Route> classes={classes!("btn")} to={Route::EventsRequest { id: event.id as u16}}>
                    { "View Event" }
                </Link<Route>>
            </div>
        </div>
    }
}

#[function_component(Upcoming)]
pub fn secure() -> Html {
    let events = events();
    html! {
        <div class="events-grid">
            { events.into_iter().map(|event| {
                html! { <SingleEventSmall event={event} /> }
            }).collect::<Html>() }
        </div>
    }
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
