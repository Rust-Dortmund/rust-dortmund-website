use crate::models::{Event, Talk};

pub fn events() -> Vec<Event> {
    let mut events = vec![
        Event {
            id: 1,
            title: "Teach and Hack - Whisperfish into Frontend".to_string(),
            description: "".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2025, 7, 23).unwrap(),
            location: "Viadee Unternehmensberatung, Sebrathweg 7, 44149 Dortmund, Dortmund, Germany".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![Talk{
                title: "Rust as a modern language in the unofficial Signal client Whisperfish",
                speaker: "Thorsten Mindrup",
                description: "With Thorsten, we explore the technical implementation of secure messaging on mobile platforms, using Whisperfish — an independent native Signal client for Sailfish OS — as a case study.",
                video_url: None,
                slides_url: None,
            }, Talk{
                title: "Modern Frontends with Rust & WebAssembly in a Typescript World",
                speaker: "Jan Vaorin",
                description: "Jan explored how Rust can complement modern frontend stacks and what this means for the future of web development. We'll dive into real-world use cases, tooling, and best practices and build our own frontend in the following workshop.",
                video_url: None,
                slides_url: None,
            }],
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
                    video_url: Some("https://youtu.be/NHv21s740ow?si=4CYrp3VKKxtReT7k".to_string()),
                    slides_url: Some("https://rust-dortmund.de/main/assets/slides/20250520_Full Stack Rust_RDM.pdf".to_string()),
                },
                Talk {
                    title: "Build Great Web Experiences with Leptos ",
                    speaker: "Ben Wishovich, Leptos Core Contributor",
                    description: "Advanced Rust programming techniques.",
                    video_url: Some("https://youtu.be/RHsIeju9QqY?si=47hhoDTNK3x1VEFp".to_string()),
                    slides_url: Some("https://rust-dortmund.de/main/assets/slides/20250520_leptos-slides_RDM.pdf".to_string()),
                },
            ],
        },
        Event {
            id: 3,
            title: "Rust Dortmund - Teach and Hack".to_string(),
            description: "".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2025, 3, 11).unwrap(),
            location:
                "Dortmund, Germany -Otto-Hahn-Straße 12,44227 Dortmund,Raum 2.063 - 2nd floor "
                    .to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![Talk {
                title: "Rust Introductino for Java Developers into an axum Workshop",
                speaker: "Timo Kösters",
                description: "Prior the workshop, Timo showed us Code Snippets in Java and Rust to compare the two languages.",
                video_url: Some("https://www.youtube.com/watch?v=iVqxD9upQPo&t=319s".to_string()),
                slides_url: None,
            }],
        },
        Event {
            id: 4,
            title: "Rust Dortmund - Talk and Connect".to_string(),
            description: "".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2024, 11, 27).unwrap(),
            location: "".to_string(),
            image_url: "".to_string(),
            talks: vec![Talk {
                title: " k23 - A secure next-generation OS through WASM",
                speaker: "Jonas Kruckenberg",
                description: "In this talk, Jonas will cover the ideas and implementation decisions behind k23, an OS designed to drastically improve security, efficiency and developer experience.",
                video_url: Some("https://youtu.be/v4HofvYXTo0?si=TxLrWmnnT7MkBRT8".to_string()),
                slides_url: Some("https://rust-dortmund.de/main/assets/slides/20241127_k23-rust-dortmund-medium_RDM.pdf".to_string()),
            }],
        },
    ];
    events.sort_by_key(|e| e.date);
    events.reverse();
    events
}
