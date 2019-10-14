
#[derive(Clone, Copy)]
pub struct Guide {
    pub slug: &'static str,
    pub menu_title: &'static str,
    pub content: &'static str,
}

impl PartialEq for Guide {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug
    }
}

pub fn guides() -> Vec<Guide> {
    vec![
        Guide {
            slug: "quickstart",
            menu_title: "Quickstart",
            content: include_str!("quickstart.md"),
        },
        Guide {
            slug: "prereqs",
            menu_title: "Prereqs",
            content: include_str!("prereqs.md"),
        },
        Guide {
            slug: "structure",
            menu_title: "Structure",
            content: include_str!("structure.md"),
        },
        Guide {
            slug: "events",
            menu_title: "Events",
            content: include_str!("events.md"),
        },
        Guide {
            slug: "components",
            menu_title: "Components",
            content: include_str!("components.md"),
        },
        Guide {
            slug: "http-requests-and-state",
            menu_title: "Http requests and state",
            content: include_str!("fetch.md"),
        },
        Guide {
            slug: "routing",
            menu_title: "Routing",
            content: include_str!("routing.md"),
        },
        Guide {
            slug: "misc-features",
            menu_title: "Misc features",
            content: include_str!("misc.md"),
        },
        Guide {
            slug: "release-and-debugging",
            menu_title: "Release and debugging",
            content: include_str!("release_and_debugging.md"),
        },
        Guide {
            slug: "complex-apps",
            menu_title: "Complex apps",
            content: include_str!("complex_apps.md"),
        },
        Guide {
            slug: "server-integration",
            menu_title: "Server integration",
            content: include_str!("server_integration.md"),
        },
        Guide {
            slug: "about",
            menu_title: "About",
            content: include_str!("about.md"),
        },
    ]
}