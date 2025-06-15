use yew::{function_component, html, Html, Properties};
use yew_router::components::Link;
use crate::app::Route;
use crate::events::events;
use crate::models::Event;

#[derive(Properties,PartialEq)]
pub struct Props {
    pub event: Event,
}



#[function_component(SingleEvent)]
pub fn secure(prop : &Props ) -> Html {
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
            <p>
                <Link<Route> to={Route::EventsRequest { id: event.id as u16 }}>
                    { "View Event" }</Link<Route>>
            </p>
            <img class="event-image" src={event.image_url.clone()} />
            <hr />
            </div>
        </div>
    }
}
#[function_component(SingleEventSmall)]
pub fn secure(prop : &Props ) -> Html {
    let is_upcoming_event = prop.event.date > chrono::Local::now().naive_local().date();
    let date_text = if is_upcoming_event {
        format!("Upcoming Event: {}", prop.event.date)
    } else {
        format!("Past Event: {}", prop.event.date)
    };
    html! {
        <div>
            <h1>{ &prop.event.title }</h1>
            <h1>{ date_text }</h1>
        </div>
    }
}

#[function_component(Upcoming)]
pub fn secure() -> Html {
    let events = events();
    events.into_iter().map(|event| {
        html! {
            <SingleEvent  event={event.clone()} />
    }
    }).collect::<Html>()
}
