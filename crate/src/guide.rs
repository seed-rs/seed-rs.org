#[derive(Clone, Copy, Debug)]
pub struct Guide {
    pub seed_version: &'static str,
    pub slug: &'static str,
    pub menu_title: &'static str,
    pub html: &'static str,
    pub lowercase_text: &'static str,
    pub prepend_menu_divider: bool,
    pub edit_url: &'static str,
}

impl PartialEq for Guide {
    fn eq(&self, other: &Self) -> bool {
        self.slug == other.slug && self.seed_version == other.seed_version
    }
}

macro_rules! guide {
    (seed_version: $seed_version:literal, slug: $slug:literal, menu_title: $menu_title:literal, file_name: $file_name:literal, prepend_menu_divider: $prepend_menu_divider:literal) => {
        Guide {
            seed_version: $seed_version,
            slug: $slug,
            menu_title: $menu_title,
            html: include_str!(concat!("../generated_guides/", concat!($seed_version, "/", $file_name, ".html"))),
            lowercase_text: include_str!(concat!("../generated_guides/", concat!($seed_version, "/", $file_name, ".txt"))),
            prepend_menu_divider: $prepend_menu_divider,
            edit_url: concat!("https://github.com/seed-rs/seed-rs.org/tree/master/crate/guides/", concat!($seed_version, "/", $file_name, ".md")),
        }
    }
}

pub fn guides() -> Vec<Guide> {
    vec![
        // ------ 0.6.0 ------
        guide!(seed_version: "0.6.0", slug: "about", menu_title: "About", file_name: "about", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "code-comparison", menu_title: "Code comparison", file_name: "code_comparison", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "quickstart", menu_title: "Quickstart", file_name: "quickstart", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "prereqs", menu_title: "Prereqs", file_name: "prereqs", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "structure", menu_title: "Structure", file_name: "structure", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "view", menu_title: "View", file_name: "view", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "events", menu_title: "Events", file_name: "events", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "http-requests-and-state", menu_title: "Http requests and state", file_name: "fetch", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "routing", menu_title: "Routing", file_name: "routing", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "misc-features", menu_title: "Misc features", file_name: "misc", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "javascript-interaction", menu_title: "Javascript interaction", file_name: "js", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "release-and-debugging", menu_title: "Release and debugging", file_name: "release_and_debugging", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "complex-apps", menu_title: "Complex apps", file_name: "complex_apps", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "server-integration", menu_title: "Server integration", file_name: "server_integration", prepend_menu_divider: false),
        guide!(seed_version: "0.6.0", slug: "support", menu_title: "Support", file_name: "support", prepend_menu_divider: true),
        guide!(seed_version: "0.6.0", slug: "changelog", menu_title: "Changelog", file_name: "changelog", prepend_menu_divider: true),
        // ------ 0.7.0 ------
        guide!(seed_version: "0.7.0", slug: "about", menu_title: "About", file_name: "about", prepend_menu_divider: false),
        // ------ 0.8.0 ------
        guide!(seed_version: "0.8.0", slug: "about", menu_title: "About", file_name: "about", prepend_menu_divider: false),
    ]
}
