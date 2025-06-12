use crate::models::Event;

pub static NEWS: [&'static str; 3] = ["First Post","Second","Third"];

pub fn events() -> Vec<Event> {
        return vec![
                Event {
                        id: 1,
                        title: "First Event".to_string(),
                        description: "Join us for our monthly Rust Dortmund meetup!".to_string(),
                        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 15).unwrap(),
                        location: "Dortmund, Germany".to_string(),
                        image_url: "https://picsum.photos/200/300".to_string(),
                },
                Event {
                        id: 2,
                        title: "Second Event".to_string(),
                        description: "Join us for our monthly Rust Dortmund meetup!".to_string(),
                        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 15).unwrap(),
                        location: "Dortmund, Germany".to_string(),
                        image_url: "https://picsum.photos/300/200".to_string(),
                }
        ]
}

