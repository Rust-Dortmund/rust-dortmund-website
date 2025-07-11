use crate::components::auth::MyApplicationMain;
use crate::components::auth::ViewUseAuth;
use crate::nav::{NavBar, Route, switch};

use yew::Html;
use yew::prelude::*;
use yew_oauth2::oauth2::{Config, OAuth2};
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

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
        <NavBar />
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
