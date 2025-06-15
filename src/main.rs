mod app;
mod news;
mod events;
mod models;

mod components {
    pub mod secure;
    pub mod home;
    pub mod events;
}
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
