
#[derive(Clone, Copy, Debug)]
pub struct Guide {
    pub slug: &'static str,
    pub menu_title: &'static str,
    pub html: &'static str,
    pub lowercase_text: &'static str,
}

impl PartialEq for Guide {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug
    }
}

pub fn guides() -> Vec<Guide> {
    vec![
        Guide {
            slug: "about",
            menu_title: "About",
            html: include_str!(concat!("../generated_guides/", "about.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "about.txt")),
        },
        Guide {
            slug: "quickstart",
            menu_title: "Quickstart",
            html: include_str!(concat!("../generated_guides/", "quickstart.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "quickstart.txt")),
        },
        Guide {
            slug: "prereqs",
            menu_title: "Prereqs",
            html: include_str!(concat!("../generated_guides/", "prereqs.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "prereqs.txt")),
        },
        Guide {
            slug: "structure",
            menu_title: "Structure",
            html: include_str!(concat!("../generated_guides/", "structure.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "structure.txt")),
        },
        Guide {
            slug: "events",
            menu_title: "Events",
            html: include_str!(concat!("../generated_guides/", "events.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "events.txt")),
        },
        Guide {
            slug: "components",
            menu_title: "Components",
            html: include_str!(concat!("../generated_guides/", "components.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "components.txt")),
        },
        Guide {
            slug: "http-requests-and-state",
            menu_title: "Http requests and state",
            html: include_str!(concat!("../generated_guides/", "fetch.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "fetch.txt")),
        },
        Guide {
            slug: "routing",
            menu_title: "Routing",
            html: include_str!(concat!("../generated_guides/", "routing.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "routing.txt")),
        },
        Guide {
            slug: "misc-features",
            menu_title: "Misc features",
            html: include_str!(concat!("../generated_guides/", "misc.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "misc.txt")),
        },
        Guide {
            slug: "release-and-debugging",
            menu_title: "Release and debugging",
            html: include_str!(concat!("../generated_guides/", "release_and_debugging.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "release_and_debugging.txt")),
        },
        Guide {
            slug: "complex-apps",
            menu_title: "Complex apps",
            html: include_str!(concat!("../generated_guides/", "complex_apps.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "complex_apps.txt")),
        },
        Guide {
            slug: "server-integration",
            menu_title: "Server integration",
            html: include_str!(concat!("../generated_guides/", "server_integration.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "server_integration.txt")),
        },
        Guide {
            slug: "changelog",
            menu_title: "Changelog",
            html: include_str!(concat!("../generated_guides/", "changelog.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "changelog.txt")),
        },
    ]
}