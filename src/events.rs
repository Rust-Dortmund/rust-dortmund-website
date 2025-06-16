use crate::models::{Event, Talk};

pub fn events() -> Vec<Event> {
    let mut events = vec![
        Event {
            id: 1,
            title: "TBD".to_string(),
            description: "".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2025, 7, 23).unwrap(),
            location: "Dortmund, Germany (more in Meetup)".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![],
        },
        Event {
            id: 2,
            title: "Talk and Connect - Fullstack".to_string(),
            description: "".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2025, 5, 20).unwrap(),
            location: "Dortmund, Germany -  Digitale Werkbank, Kanalstraße 25, 44147 Dortmund"
                .to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![
                Talk {
                    title: "Beyond Javascript -- Writing a Web Application Full Stack with Rust",
                    speaker: "Goetz Markgraf, Consulant Codecentric AG",
                    description: "Advanced Rust programming techniques.",
                    video_url: None,
                    slides_url: None,
                },
                Talk {
                    title: "Build Great Web Experiences with Leptos ",
                    speaker: "Ben Wishovich, Leptos Core",
                    description: "Advanced Rust programming techniques.",
                    video_url: None,
                    slides_url: None,
                },
            ],
        },
    ];
    events.sort_by_key(|e| e.date);
    events.reverse();
    events
}
