
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
            content: include_str!(concat!("../guides/generated_html/", "quickstart.html")),
        },
        Guide {
            slug: "prereqs",
            menu_title: "Prereqs",
            content: include_str!(concat!("../guides/generated_html/", "prereqs.html")),
        },
        Guide {
            slug: "structure",
            menu_title: "Structure",
            content: include_str!(concat!("../guides/generated_html/", "structure.html")),
        },
        Guide {
            slug: "events",
            menu_title: "Events",
            content: include_str!(concat!("../guides/generated_html/", "events.html")),
        },
        Guide {
            slug: "components",
            menu_title: "Components",
            content: include_str!(concat!("../guides/generated_html/", "components.html")),
        },
        Guide {
            slug: "http-requests-and-state",
            menu_title: "Http requests and state",
            content: include_str!(concat!("../guides/generated_html/", "fetch.html")),
        },
        Guide {
            slug: "routing",
            menu_title: "Routing",
            content: include_str!(concat!("../guides/generated_html/", "routing.html")),
        },
        Guide {
            slug: "misc-features",
            menu_title: "Misc features",
            content: include_str!(concat!("../guides/generated_html/", "misc.html")),
        },
        Guide {
            slug: "release-and-debugging",
            menu_title: "Release and debugging",
            content: include_str!(concat!("../guides/generated_html/", "release_and_debugging.html")),
        },
        Guide {
            slug: "complex-apps",
            menu_title: "Complex apps",
            content: include_str!(concat!("../guides/generated_html/", "complex_apps.html")),
        },
        Guide {
            slug: "server-integration",
            menu_title: "Server integration",
            content: include_str!(concat!("../guides/generated_html/", "server_integration.html")),
        },
        Guide {
            slug: "about",
            menu_title: "About",
            content: include_str!(concat!("../guides/generated_html/", "about.html")),
        },
    ]
}