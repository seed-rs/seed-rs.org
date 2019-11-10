#[derive(Clone, Copy, Debug)]
pub struct Guide {
    pub slug: &'static str,
    pub menu_title: &'static str,
    pub html: &'static str,
    pub lowercase_text: &'static str,
    pub prepend_menu_divider: bool,
    pub edit_url: &'static str,
}

impl PartialEq for Guide {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug
    }
}

// @TODO 1 macro or more macros (https://stackoverflow.com/a/35159310)
pub fn guides() -> Vec<Guide> {
    vec![
        Guide {
            slug: "about",
            menu_title: "About",
            html: include_str!(concat!("../generated_guides/", "about.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "about.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "about.md"),
        },
        Guide {
            slug: "quickstart",
            menu_title: "Quickstart",
            html: include_str!(concat!("../generated_guides/", "quickstart.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "quickstart.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "quickstart.md"),
        },
        Guide {
            slug: "prereqs",
            menu_title: "Prereqs",
            html: include_str!(concat!("../generated_guides/", "prereqs.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "prereqs.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "prereqs.md"),
        },
        Guide {
            slug: "structure",
            menu_title: "Structure",
            html: include_str!(concat!("../generated_guides/", "structure.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "structure.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "structure.md"),
        },
        Guide {
            slug: "events",
            menu_title: "Events",
            html: include_str!(concat!("../generated_guides/", "events.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "events.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "events.md"),
        },
        Guide {
            slug: "components",
            menu_title: "Components",
            html: include_str!(concat!("../generated_guides/", "components.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "components.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "components.md"),
        },
        Guide {
            slug: "http-requests-and-state",
            menu_title: "Http requests and state",
            html: include_str!(concat!("../generated_guides/", "fetch.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "fetch.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "fetch.md"),
        },
        Guide {
            slug: "routing",
            menu_title: "Routing",
            html: include_str!(concat!("../generated_guides/", "routing.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "routing.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "routing.md"),
        },
        Guide {
            slug: "misc-features",
            menu_title: "Misc features",
            html: include_str!(concat!("../generated_guides/", "misc.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "misc.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "misc.md"),
        },
        Guide {
            slug: "release-and-debugging",
            menu_title: "Release and debugging",
            html: include_str!(concat!("../generated_guides/", "release_and_debugging.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "release_and_debugging.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "release_and_debugging.md"),
        },
        Guide {
            slug: "complex-apps",
            menu_title: "Complex apps",
            html: include_str!(concat!("../generated_guides/", "complex_apps.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "complex_apps.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "complex_apps.md"),
        },
        Guide {
            slug: "server-integration",
            menu_title: "Server integration",
            html: include_str!(concat!("../generated_guides/", "server_integration.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "server_integration.txt")),
            prepend_menu_divider: false,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "server_integration.md"),
        },
        Guide {
            slug: "changelog",
            menu_title: "Changelog",
            html: include_str!(concat!("../generated_guides/", "changelog.html")),
            lowercase_text: include_str!(concat!("../generated_guides/", "changelog.txt")),
            prepend_menu_divider: true,
            edit_url: concat!("https://github.com/MartinKavik/seed-rs.org/tree/master/crate/guides/", "changelog.md"),
        },
    ]
}
