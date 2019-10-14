
pub struct Article {
    pub menu_title: &'static str,
    pub slug: &'static str,
    pub content: &'static str,
}

pub fn articles() -> Vec<Article> {
    vec![
        Article {
            menu_title: "Quickstart",
            slug: "quickstart",
            content: include_str!("quickstart.md"),
        },
        Article {
            menu_title: "Prereqs",
            slug: "prereqs",
            content: include_str!("prereqs.md"),
        },
        Article {
            menu_title: "Structure",
            slug: "structure",
            content: include_str!("structure.md"),
        },
        Article {
            menu_title: "Events",
            slug: "events",
            content: include_str!("events.md"),
        },
        Article {
            menu_title: "Components",
            slug: "components",
            content: include_str!("components.md"),
        },
        Article {
            menu_title: "Http requests and state",
            slug: "http-requests-and-state",
            content: include_str!("fetch.md"),
        },
        Article {
            menu_title: "Routing",
            slug: "routing",
            content: include_str!("routing.md"),
        },
        Article {
            menu_title: "Misc features",
            slug: "misc-features",
            content: include_str!("misc.md"),
        },
        Article {
            menu_title: "Release and debugging",
            slug: "release-and-debugging",
            content: include_str!("release_and_debugging.md"),
        },
        Article {
            menu_title: "Complex apps",
            slug: "complex-apps",
            content: include_str!("complex_apps.md"),
        },
        Article {
            menu_title: "Server integration",
            slug: "server-integration",
            content: include_str!("server_integration.md"),
        },
        Article {
            menu_title: "About",
            slug: "about",
            content: include_str!("about.md"),
        },
    ]
}