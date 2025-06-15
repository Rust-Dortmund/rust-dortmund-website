use crate::models::{Event,Talk};

pub static NEWS: [&'static str; 3] = ["First Post","Second","Third"];

pub fn events() -> Vec<Event> {
        let mut events =  vec![
                Event {
                        id: 1,
                        title: "First Event".to_string(),
                        description: "Join us for our monthly Rust Dortmund meetup!".to_string(),
                        date: chrono::NaiveDate::from_ymd_opt(2026, 10, 15).unwrap(),
                        location: "Dortmund, Germany".to_string(),
                        image_url: "https://picsum.photos/200/300".to_string(),
                        talks: vec![Talk{
                                title: "First Talk".to_string(),
                                speaker: "John Doe",
                                description: "An introduction to Rust programming.".to_string(),
                        }],
                },
                Event {
                        id: 2,
                        title: "Second Event".to_string(),
                        description: "Join us for our monthly Rust Dortmund meetup!".to_string(),
                        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 15).unwrap(),
                        location: "Dortmund, Germany".to_string(),
                        image_url: "https://picsum.photos/300/200".to_string(),
                        talks: vec![Talk{
                                title: "Second Talk".to_string(),
                                speaker: "Jane Smith",
                                description: "Advanced Rust programming techniques.".to_string(),
                        }],
                }
        ];
        events.sort_by_key( |e| e.date);
        events.reverse();
        return events;
}

