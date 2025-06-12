pub struct Event {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub date: chrono::NaiveDate,
    pub location: String,
    pub image_url: String,
}

pub struct News {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub date: chrono::NaiveDate,
    pub author: String,
}