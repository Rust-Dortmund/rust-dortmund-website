mod app;
mod events;
mod models;
mod components {
    pub mod events;
    pub mod home;
    pub mod secure;
    pub mod showcase;
}
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
