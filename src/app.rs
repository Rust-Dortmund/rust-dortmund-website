use crate::components::auth::MyApplicationMain;
use crate::components::auth::ViewUseAuth;
use crate::components::events::{RequestTest, SingleEvent};
use crate::components::showcase::Showcase;
use crate::components::{events::Upcoming, home::Home};
use crate::events::events;
use yew::prelude::*;
use yew::Html;
use yew_oauth2::oauth2::{Config, OAuth2};
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

static IMPRESSUM: &str = "Impressum";

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
        { IMPRESSUM }
            </a>
        </h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let config = Config::new(
        "363680698182-7e65kptgveqmtsmnd62t74tcopo28sl5.apps.googleusercontent.com",
        "https://accounts.google.com/o/oauth2/v2/auth",
    );

    html! {
          <BrowserRouter>
         <OAuth2
        {config}
        scopes={vec!["https://www.googleapis.com/auth/userinfo.email".to_string(),"https://www.googleapis.com/auth/userinfo.profile".to_string()]}
        >
        <section class="app">
        <nav class="header topnav" id="myTopnav">
        <Link<Route> classes={classes!("active")}  to={Route::Home}>{ "Home" }</Link<Route>>
        <Link<Route> to={Route::UpcomingEventListRequest}>{ "Events" }</Link<Route>>
        <Link<Route> to={Route::Impressum}>{ "Impressum" }</Link<Route>>
        <Link<Route> to={Route::Showcase}>{ "Showcase" }</Link<Route>>
        <MyApplicationMain  />
        <a class="icon" id="close"> {"MENU"}</a>
        </nav>
        <div class="body">
        <main>
          <UseAuthentication<ViewUseAuth>>
            <ViewUseAuth/>
          </UseAuthentication<ViewUseAuth>>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </main>
        </div>
        </section>
        // Rustify this at some point
        </OAuth2>
        </BrowserRouter>
    }
}
