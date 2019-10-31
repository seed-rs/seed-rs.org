// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(clippy::used_underscore_binding)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::enum_glob_use)]

mod generated;
mod page;
mod guide;

use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{events::Listener, prelude::*, *};
use Visibility::*;
use guide::Guide;
use std::{borrow::Cow, convert::TryFrom, fmt};
use std::convert::TryInto;
use crate::Route::Root;

const TITLE_SUFFIX: &str = "Kavik.cz";
// https://mailtolink.me/
const MAIL_TO_KAVIK: &str = "mailto:martin@kavik.cz?subject=Something%20for%20Martin&body=Hi!%0A%0AI%20am%20Groot.%20I%20like%20trains.";
const MAIL_TO_HELLWEB: &str =
    "mailto:martin@hellweb.app?subject=Hellweb%20-%20pain&body=Hi!%0A%0AI%20hate";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn toggle(&mut self) {
        *self = match self {
            Visible => Hidden,
            Hidden => Visible,
        }
    }
}

// ------ ------
//     Model
// ------ ------

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

pub struct Model {
    pub page: Page,
    pub scroll_history: ScrollHistory,
    pub guide_list_visibility: Visibility,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,
    pub guides: Vec<Guide>,
    pub search_query: String,
    pub matched_guides: Vec<Guide>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Guide(Guide),
    // @TODO remove about
    About,
    NotFound,
}

impl Page {
    pub fn to_href(self) -> String {
        match self {
            Self::Guide(guide) => format!("/guide/{}", guide.slug),
            Self::About => "/about".into(),
            Self::NotFound => "/404".into(),
        }
    }

    pub fn from_route_and_replace_history(route: &Route, guides: &[Guide]) -> Self {
        match route {
            Route::Root => {
                match guides.first() {
                    Some(guide) => {
                        let page = Page::Guide(*guide);
                        window().history().unwrap().replace_state_with_url(&JsValue::NULL, "", Some(&page.to_href()));
                        page
                    },
                    None => Page::NotFound,
                }
            },
            Route::Guide(slug) => {
                match guides.iter().find(|guide| guide.slug == slug) {
                    Some(guide) => Page::Guide(*guide),
                    None => Page::NotFound,
                }
            },
            Route::About => Page::About,
            Route::Unknown => Page::NotFound,
        }
    }
}

pub fn previous_guide<'a>(selected_guide: &Guide, guides: &'a [Guide]) -> Option<&'a Guide> {
    let selected_guide_index =
        guides.iter().position(|guide| guide == selected_guide)?;

    selected_guide_index.checked_sub(1).and_then(|index| guides.get(index))
}

pub fn next_guide<'a>(selected_guide: &Guide, guides: &'a [Guide]) -> Option<&'a Guide> {
    let selected_guide_index =
        guides.iter().position(|guide| guide == selected_guide)?;

    selected_guide_index.checked_add(1).and_then(|index| guides.get(index))
}

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Init<Model> {
    // @TODO: Seed can't hydrate prerendered html (yet).
    // https://github.com/David-OConnor/seed/issues/223
    if let Some(mount_point_element) = document().get_element_by_id("app") {
        mount_point_element.set_inner_html("");
    }

    orders.send_msg(Msg::UpdatePageTitle);

    let guides = guide::guides();

    Init::new_with_url_handling(Model {
        page: Page::from_route_and_replace_history(&url.into(), &guides),
        scroll_history: ScrollHistory::new(),
        guide_list_visibility: Hidden,
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
        guides,
        search_query: "".to_string(),
        matched_guides: vec![],
    }, UrlHandling::None)
}

fn is_in_prerendering() -> bool {
    let user_agent =
        window().navigator().user_agent().expect("cannot get user agent");

    user_agent == USER_AGENT_FOR_PRERENDERING
}

// ------ ------
//    Routes
// ------ ------

pub fn routes(url: Url) -> Option<Msg> {
    // Urls which start with `static` are files => treat them as external links.
    if url.path.starts_with(&[STATIC_PATH.into()]) {
        return None;
    }
    Some(Msg::RouteChanged(url.into()))
}

#[derive(Clone)]
pub enum Route {
    Root,
    Guide(String),
    About,
    Unknown,
}

impl From<Url> for Route {
    fn from(url: Url) -> Self {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Route::Root,
            Some("about") => Route::About,
            Some("guide") => path.next().map(Route::Guide).unwrap_or(Route::Unknown),
            _ => Route:: Unknown,
        }
    }
}

impl Route {
    pub fn path(&self) -> Vec<&str> {
        match self {
            Route::Root => vec![],
            Route::About => vec!["about"],
            Route::Guide(slug) => vec!["guide", slug.as_str()],
            Route::Unknown => vec!["404"],
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path().join("/"))
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    RouteChanged(Route),
    UpdatePageTitle,
    ScrollToTop,
    Scrolled(i32),
    ToggleGuideList,
    HideGuideList,
    ToggleMenu,
    HideMenu,
    SearchQueryChanged(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(route) => {
            model.page = Page::from_route_and_replace_history(&route, &model.guides);
            orders.send_msg(Msg::ScrollToTop);
            orders.send_msg(Msg::UpdatePageTitle);
        },
        Msg::UpdatePageTitle => {
            let title = match model.page {
                Page::Guide(_) => TITLE_SUFFIX.to_owned(),
                Page::About => format!("About - {}", TITLE_SUFFIX),
                Page::NotFound => format!("404 - {}", TITLE_SUFFIX),
            };
            document().set_title(&title);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::Scrolled(position) => {
            *model.scroll_history.push_back() = position;
        },
        Msg::ToggleGuideList => model.guide_list_visibility.toggle(),
        Msg::HideGuideList => {
            model.guide_list_visibility = Hidden;
        },
        Msg::ToggleMenu => model.menu_visibility.toggle(),
        Msg::HideMenu => {
            model.menu_visibility = Hidden;
        },
        Msg::SearchQueryChanged(query) => {
            model.matched_guides = search(&model.guides, &query);
            model.search_query = query;
        }
    }
}

fn search(guides: &[Guide], query: &str) -> Vec<Guide> {
    if query.is_empty() {
        return Vec::new()
    }

    let query = query.to_lowercase();

    guides.iter().filter_map(|guide|{
        if guide.lowercase_text.contains(&query) {
            Some(*guide)
        } else {
            None
        }
    }).collect()
}

// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0

pub fn view(model: &Model) -> impl View<Msg> {
        div![
            class![
                C.min_h_screen,
                C.tracking_wider,
                C.font_body,
            ],
            match model.page {
                Page::Guide(guide) => page::guide::view(&guide, model).els(),
                Page::About => page::about::view().els(),
                Page::NotFound => page::not_found::view().els(),
            },
            page::partial::header::view(model).els(),
        ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

// ------ ------
// Window Events
// ------ ------

pub fn window_events(_: &Model) -> Vec<Listener<Msg>> {
    vec![raw_ev(Ev::Scroll, |_| {
        // Some browsers use `document.body.scrollTop`
        // and other ones `document.documentElement.scrollTop`.
        let mut position = body().scroll_top();
        if position == 0 {
            position = document()
                .document_element()
                .expect("cannot get document element")
                .scroll_top()
        }
        Msg::Scrolled(position)
    })]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::build(init, update, view)
        .routes(routes)
        .window_events(window_events)
        .finish()
        .run();

    log!("App started.");
}
