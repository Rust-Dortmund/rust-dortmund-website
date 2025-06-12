use yew::{function_component, html, Html};
use crate::events::events;

#[function_component(Upcoming)]
pub fn secure() -> Html {
    let events = events();
    events.into_iter().map(|event| {
        html! {
            <div class="event" key={event.id}>
                <h2>{ &event.title }</h2>
                <p>{ &event.description }</p>
                <p>{ format!("Date: {}", event.date) }</p>
                <p>{ format!("Location: {}", event.location) }</p>
            <img class="event-image" src={event.image_url} />
            <hr />
            </div>
    }
    }).collect::<Html>()
}