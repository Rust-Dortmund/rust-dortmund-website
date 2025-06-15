use yew::prelude::*;
use yew_router::prelude::*;
use crate::news::NEWS;
use crate::components::{secure::Secure,home::Home};
use crate::components::upcoming::Upcoming;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[at("/news")]
    NewsListRequest,
    #[at("/news/:id")]
    NewsRequest { id: u16 },
    #[at("/past-events")]
    PastEventListRequest,
    #[at("/upcoming-events")]
    UpcomingEventListRequest,
    #[at("/event/:id")]
    PastEventsRequest { id: u16 },
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home /> },
        Route::Secure => html! {
            <Secure />
        },
        Route::NewsRequest { id } => html! {
            <h1>{ format!("News {}", NEWS.get(id as usize).unwrap_or(&"Unknown News")) }</h1>
        },
        Route::NewsListRequest => html! {
            NEWS.into_iter().map(|name| {
                html!{<div key={name}>{ format!("News: {}", name) }</div>}
            }).collect::<Html>()
        },
        Route::PastEventsRequest { id } => html! {
            <h1>{ format!("Past Event {}", id) }</h1>
        },
        Route::PastEventListRequest => html! {
            <h1>{ "Past Events" }</h1>
        },
        Route::UpcomingEventListRequest => html! {
            <Upcoming />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Test => html!{<h1>{ "Test Page" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
          <BrowserRouter>
        <section class="app">
        <nav class="header topnav" id="myTopnav">
        <Link<Route> classes={classes!("active")}  to={Route::Home}>{ "Home" }</Link<Route>>
        <Link<Route> to={Route::UpcomingEventListRequest}>{ "Events" }</Link<Route>>
        <Link<Route> to={Route::NewsListRequest}>{ "News" }</Link<Route>>
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

