mod app;
mod events;
mod models;
mod news;

mod components {
    pub mod events;
    pub mod home;
    pub mod secure;
}
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
