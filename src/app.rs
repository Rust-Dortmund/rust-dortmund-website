use crate::components::events::{RequestTest, SingleEvent};
use crate::components::showcase::Showcase;
use crate::components::{events::Upcoming, home::Home};
use crate::events::events;
use gloo::storage::Storage;
use gloo_net::http::Request;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_oauth2::oauth2::{use_auth_agent, Config, OAuth2};
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

static IMPRESSUM: &'static str = "Impressum";

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
        Route::Home => html! {<Suspense> <Home /> </Suspense>},
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
        "https://accounts.google.com/o/oauth2/token",
        //"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",
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
        <a class="icon" id="close"> {" MENU"}</a>
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
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or(dummy())]
    pub auth: Authentication,
}

impl UseAuthenticationProperties for Props {
    fn set_authentication(&mut self, auth: Authentication) {
        self.auth = auth;
    }
}

fn dummy() -> Authentication {
    Authentication {
        access_token: "".to_string(),
        refresh_token: None,
        expires: None,
        //client_secret: None,
    }
}

#[function_component(ViewUseAuth)]
pub fn view_use_auth(props: &Props) -> Html {
    html!(
        <>
            <h2> { "Use authentication example"} </h2>
            <Suspense fallback={html! {<h1>{ "Loading..." }</h1>}}>
            <ViewAuthInfo auth={props.auth.clone()} />
        </Suspense>
        </>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AuthProps {
    pub auth: Authentication,
}

#[function_component(ViewAuthInfo)]
pub fn view_auth(props: &AuthProps) -> HtmlResult {
    let token = props.auth.access_token.clone();
    const URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";
    let res = use_future(|| async move {
        Request::get(URL)
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?
            .text()
            .await
    })?;

    let result_html = match *res {
        Ok(ref res) => html! { res },
        Err(ref failure) => html! {
                { format!("Error fetching data: {}", failure) }
        },
    };
    Ok(html!(
        <dl>
            <dt> { "Context" } </dt>
            <dd>
                <code><pre>
                    { format!("{:#?}", props.auth) }
                    { result_html }
                </pre></code>
            </dd>
        </dl>
    ))
}
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContextProps {
    pub auth: OAuth2Context,
}

#[function_component(MyApplicationMain)]
fn my_app_main() -> Html {
    let agent = use_auth_agent().expect("Must be nested inside an OAuth2 component");
    let agent = agent.clone();

    let login = use_callback(agent.clone(), |_, agent| {
        //set scope
        let url = "http://localhost:8080/".parse().unwrap();
        let mut opts = LoginOptions::default();
        opts.redirect_url = Some(url);
        let _ = agent.start_login_opts(opts).unwrap();
    });
    let logout = use_callback(agent, |_, agent| {
        let _ = agent.logout();
    });

    html!(
      <>
        <Failure><FailureMessage/></Failure>
        <Authenticated>
          <button onclick={logout}>{ "Logout" }</button>
        </Authenticated>
        <NotAuthenticated>
          <button onclick={login}>{ "Login" }</button>

        </NotAuthenticated>
      </>
    )
}
