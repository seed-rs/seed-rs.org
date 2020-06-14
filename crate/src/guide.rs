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
        guide!(seed_version: "0.7.0", slug: "getting_started", menu_title: "Getting Started", file_name: "getting_started", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "rust", menu_title: "Rust", file_name: "rust", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "new_app", menu_title: "New App", file_name: "new_app", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "app_1_counter", menu_title: "App 1: Counter", file_name: "app_1_counter", prepend_menu_divider: true),
        guide!(seed_version: "0.7.0", slug: "use", menu_title: "Use", file_name: "use", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "model", menu_title: "Model", file_name: "model", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "init", menu_title: "Init", file_name: "init", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "msg", menu_title: "Msg", file_name: "msg", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "update", menu_title: "Update", file_name: "update", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "view", menu_title: "View", file_name: "view", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "element_macros", menu_title: "Element Macros", file_name: "element_macros", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "attributes", menu_title: "Attributes", file_name: "attributes", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "event_handlers", menu_title: "Event Handlers", file_name: "event_handlers", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "start", menu_title: "Start", file_name: "start", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "app_2_todomvc", menu_title: "App 2: TodoMVC", file_name: "app_2_todomvc", prepend_menu_divider: true),
        guide!(seed_version: "0.7.0", slug: "todomvc_model", menu_title: "Model", file_name: "todomvc_model", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "todomvc_msg", menu_title: "Msg", file_name: "todomvc_msg", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "todomvc_project_setup", menu_title: "Project Setup", file_name: "todomvc_project_setup", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "todomvc_view", menu_title: "View", file_name: "todomvc_view", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "todomvc_update", menu_title: "Update", file_name: "todomvc_update", prepend_menu_divider: false),
        guide!(seed_version: "0.7.0", slug: "todomvc_local_storage", menu_title: "LocalStorage", file_name: "todomvc_local_storage", prepend_menu_divider: false),
        // ------ 0.8.0 ------
        guide!(seed_version: "0.8.0", slug: "about", menu_title: "About", file_name: "about", prepend_menu_divider: false),
    ]
}
