use crate::components::events::{RequestTest, SingleEvent};
use crate::components::showcase::Showcase;
use crate::components::{events::Upcoming, home::Home};
use crate::events::events;
use crate::models::State;
use crate::news::NEWS;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::use_store;

static IMPRESSUM: &'static str = "Testimpressum In Dortmund";

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home /> },
        Route::Showcase => html! {<h1><Showcase /></h1> },
        Route::NewsRequest { id } => html! {
            <h1>{ format!("News {}", NEWS.get(id as usize).unwrap_or(&"Unknown News")) }</h1>
        },
        Route::NewsListRequest => html! {
            NEWS.into_iter().map(|name| {
                html!{<div key={name}>{ format!("News: {}", name) }</div>}
            }).collect::<Html>()
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
        Route::Impressum => html! {<h1>{ IMPRESSUM }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let (state, _) = use_store::<State>();
    let mut username = state.user.clone().unwrap_or("Guest".to_string());
    if state.count > 0 {
        username = "LOGGED IN USER".to_string();
    }
    html! {
          <BrowserRouter>
        <section class="app">
        <nav class="header topnav" id="myTopnav">
        <Link<Route> classes={classes!("active")}  to={Route::Home}>{ "Home" }</Link<Route>>
        <Link<Route> to={Route::UpcomingEventListRequest}>{ "Events" }</Link<Route>>
        <Link<Route> to={Route::Impressum}>{ "Impressum" }</Link<Route>>
        <Link<Route> to={Route::Showcase}>{ "Showcase" }</Link<Route>>
        {state.count}
        {username}
        <a class="icon" id="close"> {" MENU"}</a>
        </nav>
        <div class="body">
        <main>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </main>
        </div>
        </section>
        // Rustify this at some point
        </BrowserRouter>
    }
}
