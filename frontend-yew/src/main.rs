mod app;
mod events;
mod models;
mod components {
    pub mod auth;
    pub mod events;
    pub mod home;
    pub mod showcase;
}
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
