use strum::IntoEnumIterator;
use yew::Html;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::events::{RequestTest, SingleEvent};
use crate::components::showcase::Showcase;
use crate::components::{events::Upcoming, home::Home};
use crate::events::events;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let root: Option<Route> = use_route();
    MainNavLinks::iter()
        .map(|link| {
            let route: Route = link.as_route();
            let is_active = root.as_ref().is_some_and(|r| *r == route);
            html! {
                <Link<Route> classes={classes!(if is_active { "active" } else { "" })} to={route}>
                    { link.to_string() }
                </Link<Route>>
            }
        })
        .collect::<Html>()
}

#[derive(Clone, Default, Debug, Eq, PartialEq, strum::EnumIter, strum::Display)]
pub enum MainNavLinks {
    #[default]
    Home,
    Events,
    Showcase,
    Impressum,
}

impl MainNavLinks {
    pub fn as_route(&self) -> Route {
        match self {
            MainNavLinks::Home => Route::Home,
            MainNavLinks::Events => Route::UpcomingEventListRequest,
            MainNavLinks::Showcase => Route::Showcase,
            MainNavLinks::Impressum => Route::Impressum,
        }
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/news")]
    NewsListRequest,
    #[at("/showcase")]
    Showcase,
    #[at("/news/:id")]
    NewsRequest { id: u16 },
    #[at("/events")]
    UpcomingEventListRequest,
    #[at("/event/:id")]
    EventsRequest { id: u16 },
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/impressum")]
    Impressum,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
        <><Home /></>},
        Route::Showcase => html! {<h1><Showcase /></h1> },
        Route::NewsRequest { id } => html! {
            <h1>{ format!("News {}",id) }</h1>
        },
        Route::NewsListRequest => html! {
            <h1>{ "News List" }</h1>
        },
        Route::EventsRequest { id } => html! {
            events().into_iter().filter(|e|{e.id == id as u32}).map(|event| {
                html! {
                    <SingleEvent  event={event} />
                }
            }).collect::<Html>()
        },
        Route::UpcomingEventListRequest => html! {
            <Upcoming />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Test => html! {<h1>
        <Suspense fallback={html! {<h1>{ "Loading..." }</h1>}}>
        <RequestTest />
        </Suspense>
        </h1> },
        Route::Impressum => html! {<h1>
            <a href="https://www.corgi.wiki/impressum" target="_blank" rel="noopener noreferrer">
        { "Impressum" }
            </a>
        </h1> },
    }
}
