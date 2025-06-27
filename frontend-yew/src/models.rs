use yew::Properties;
use yewdux::Store;

#[derive(Properties, Clone, PartialEq)]
pub struct Event {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub date: chrono::NaiveDate,
    pub location: String,
    pub image_url: String,
    pub talks: Vec<Talk>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct News {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub date: chrono::NaiveDate,
    pub author: String,
}
#[derive(Properties, Clone, PartialEq)]
pub(crate) struct Talk {
    pub title: &'static str,
    pub speaker: &'static str,
    pub description: &'static str,
    pub video_url: Option<String>,
    pub slides_url: Option<String>,
}

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    pub count: u32,
    pub user: Option<String>,
}
