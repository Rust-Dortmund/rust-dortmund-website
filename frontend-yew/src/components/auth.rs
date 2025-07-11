use gloo::net::http::Request;
use yew::suspense::use_future;
use yew::{Html, HtmlResult, Properties, Suspense, function_component, html, use_callback};
use yew_oauth2::agent::{LoginOptions, OAuth2Client, OAuth2Operations};
use yew_oauth2::components::UseAuthenticationProperties;
use yew_oauth2::components::context::{Agent, use_auth_agent};
use yew_oauth2::context::{Authentication, OAuth2Context};
use yew_oauth2::prelude::{Authenticated, NotAuthenticated};

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
    }
}

#[function_component(ViewUseAuth)]
pub fn view_use_auth(props: &Props) -> Html {
    html!(
        <>
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
    #[cfg(not(feature = "auth"))]
    {
        return Ok(html! {});
    }
    #[cfg(feature = "auth")]
    {
        let token = props.auth.access_token.clone();

        const URL: &str = "https://www.googleapis.com/oauth2/v3/userinfo";
        let res = use_future(|| async move {
            Request::get(URL)
                .header("Authorization", &format!("Bearer {token}"))
                .send()
                .await?
                .text()
                .await
        })?;

        let _ = match *res {
            Ok(ref res) => {
                let json: serde_json::Value = serde_json::from_str(res).unwrap_or_default();
                let name = json
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("Unknown");
                return Ok(html! {
                    <p>{format!("You are Authenticated {name}")}</p>
                });
            }
            Err(ref failure) => html! {
                    { format!("Error fetching data: {}", failure) }
            },
        };
        Ok(html!(
            <>
            </>
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContextProps {
    pub auth: OAuth2Context,
}

#[cfg(not(feature = "auth"))]
#[function_component(MyApplicationMain)]
pub fn my_app_main() -> Html {
    html!(
      <>
      </>
    )
}

#[cfg(feature = "auth")]
#[function_component(MyApplicationMain)]
pub fn my_app_main() -> Html {
    let agent: Agent<OAuth2Client> =
        use_auth_agent().expect("Must be nested inside an OAuth2 component");
    let agent = agent.clone();

    let login = use_callback(agent.clone(), |_, agent| {
        //set scope
        let url = "http://localhost:8080/".parse().unwrap();
        let mut opts = LoginOptions::default();
        opts.redirect_url = Some(url);
        agent.start_login_opts(opts).expect("Failed to start login");
    });
    let logout = use_callback(agent, |_, agent| {
        let _ = agent.logout();
    });

    html!(
      <>
        <Authenticated>
          <a onclick={logout}>{ "Logout" }</a>
        </Authenticated>
        <NotAuthenticated>
          <a onclick={login}>{ "Login" }</a>
        </NotAuthenticated>
      </>
    )
}
