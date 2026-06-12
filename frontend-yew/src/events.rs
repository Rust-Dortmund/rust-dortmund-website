use crate::models::{Event, Talk};

pub fn events() -> Vec<Event> {
    let mut events = vec![
        Event {
            id: 10,
            title: "Rust Dortmund Meetup - Agentic Programming - May".to_string(),
            description: "A hybrid meetup focused on Agentic Programming: how AI coding agents and Rust fit together, and how strong test suites keep agents in check.".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 5, 26).unwrap(),
            location: "Conciso GmbH - Workgarden, Pariser Bogen 7, 44269 Dortmund, Germany".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![
                Talk {
                    title: "Rust for Agentic Programming - Workflows and Guardrails",
                    speaker: "Tim Janus",
                    description: "Tim explores agentic programming through MeetNTalk, the event-management web app of Rust Dortmund, built with AI tools like Claude, Codex, and GitHub Copilot. A comprehensive test suite acts as guardrails for the agents, and Rust's strict typing and compile-time safety patterns (e.g. the newtype idiom) prove especially well-suited for this paradigm.",
                    video_url: None,
                    slides_url: None,
                },
                Talk {
                    title: "Can Your Tests Catch This? Practical Mutation Testing for Rust Programs with mutest-rs",
                    speaker: "Zalán Bálint Lévai",
                    description: "Zalán introduces mutation testing: intentionally injecting faults to check whether your tests detect them, uncovering weak tests and untested behaviour. He shows real testing gaps and lurking bugs found in open-source Rust projects and how mutest-rs builds on the Rust compiler to make mutation testing practical.",
                    video_url: None,
                    slides_url: None,
                },
            ],
        },
        Event {
            id: 9,
            title: "Rust Dortmund Meetup - Intro to Embedded Rust - March".to_string(),
            description: "A meetup focused on Embedded Rust, run as a hybrid event together with the Rust and C++ Dragons (formerly Rust Cardiff). Followed up by a 4h on-site hands-on workshop on 2026-04-29.".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 3, 18).unwrap(),
            location: "Adesso New Building, Freie Vogel Straße 383, 44269 Dortmund, Germany".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![
                Talk {
                    title: "Intro to Embedded Rust",
                    speaker: "Domenic Quirl",
                    description: "Domenic looks at embedded development in general and why Rust is a good fit for it: starting from how a microcontroller is structured, he shows what to consider when writing code and why the Rust embedded ecosystem handles the complex bits so you can focus on the fun parts.",
                    video_url: None,
                    slides_url: None,
                },
                Talk {
                    title: "A Soft Intro to the Embassy Framework",
                    speaker: "Björn Barwinski",
                    description: "Björn tackles the growing state-tracking mess in embedded code by handling inputs and actions concurrently with the embassy framework for async tasks on embedded devices, explains why async is actually easier to understand on embedded than with std, and compares the approach to alternatives.",
                    video_url: None,
                    slides_url: None,
                },
            ],
        },
        Event {
            id: 8,
            title: "Rust Dortmund Meetup - January 2026".to_string(),
            description: "An evening of learning, collaboration, and community building.".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2026, 1, 28).unwrap(),
            location: "viadee Unternehmensberatung AG, Sebrathweg 7, 44149 Dortmund, Germany".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![Talk {
                title: "Finite Element Simulation with Rust",
                speaker: "Dr. Henrik Stromberg (Dr.Q)",
                description: "Henrik has worked with FEM simulation software as a mechanical engineer for over 10 years. At Dr.Q he is leading the development of an FEM solution built from scratch in Rust.",
                video_url: None,
                slides_url: None,
            }],
        },
        Event {
            id: 7,
            title: "Rust Dortmund Meetup - December 2025".to_string(),
            description: "An evening of learning, collaboration, and community building, including lightning talks.".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2025, 12, 8).unwrap(),
            location: "viadee Unternehmensberatung AG, Sebrathweg 7, 44149 Dortmund, Germany".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![
                Talk {
                    title: "Decompiling Mario",
                    speaker: "Adrian Kathagen",
                    description: "Adrian takes us into the world of decompilation using Mario as a case study.",
                    video_url: None,
                    slides_url: None,
                },
                Talk {
                    title: "Gamedev with Rust - Macroquad and Bevy",
                    speaker: "Tim Janus",
                    description: "Tim gives an introduction to game development in Rust with the Macroquad and Bevy frameworks.",
                    video_url: None,
                    slides_url: None,
                },
            ],
        },
        Event {
            id: 6,
            // TODO: Talk details for this event are not publicly available on
            // meetup.com (the full past-events list is behind a login).
            // Please fill in talks/description from your own records.
            title: "Rust Dortmund Meetup - October 2025".to_string(),
            description: "".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2025, 10, 29).unwrap(),
            location: "adesso SE, Adessoplatz 1, 44269 Dortmund, Germany".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![],
        },
        Event {
            id: 5,
            title: "Rust for Safety-Critical Software-Development and other high potential Use Cases".to_string(),
            description: "This Rust Dortmund event is part of the digital week in Dortmund, focusing on safety-critical software development in domains such as automotive, aviation, and process industries, as well as other high-potential Rust use-cases like Serverless.".to_string(),
            date: chrono::NaiveDate::from_ymd_opt(2025, 9, 3).unwrap(),
            location: "Digitale Werkbank (Industrie Campus - Dortmund Hafen), Kanalstraße 25, 44147 Dortmund, Germany".to_string(),
            image_url: "./assets/janPizza.jpg".to_string(),
            talks: vec![
                Talk {
                    title: "Rust in the automotive industry: Developing safety-critical applications efficiently and qualitatively",
                    speaker: "Christof Petig",
                    description: "Christof highlights the advantages of Rust for developing safety-critical applications through several examples, showing how Rust enables higher quality in less time. He presents methods and technologies that make it possible to build safety-critical systems even with mixed programming languages.",
                    video_url: None,
                    slides_url: None,
                },
                Talk {
                    title: "Why Serverless is the Best Choice for Your Next Greenfield Project",
                    speaker: "Florian Lenz",
                    description: "Florian explains the 'Serverless First' approach, showing how it enables teams to focus on features instead of infrastructure. He illustrates the benefits and tradeoffs of serverless with real-world examples like LEGO.com, Coca-Cola vending machines, and Amazon Prime Video.",
                    video_url: None,
                    slides_url: None,
                },
            ],
        },
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
