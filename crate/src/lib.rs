// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]

#![allow(clippy::used_underscore_binding)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::enum_glob_use)]

mod generated;
mod page;
mod guide;

use generated::css_classes::C;
use seed::{prelude::*, *};
use Visibility::*;
use guide::Guide;
use std::fmt;
use serde::{Serialize, Deserialize};
use serde_json;
use std::convert::identity;

const STORAGE_KEY: &str = "seed";
const TITLE_SUFFIX: &str = "Seed";
const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";

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

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    mode: Mode,
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    pub page: Page,
    pub guide_list_visibility: Visibility,
    pub menu_visibility: Visibility,
    pub in_prerendering: bool,
    pub guides: Vec<Guide>,
    pub search_query: String,
    pub matched_guides: Vec<Guide>,
    pub mode: Mode,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Light,
    Dark
}

impl Mode {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Light
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Guide { guide: Guide, show_intro: bool },
    NotFound,
}

impl Page {
    pub fn to_href(self) -> String {
        match self {
            Self::Guide { guide, ..} => format!("/guide/{}", guide.slug),
            Self::NotFound => "/404".into(),
        }
    }

    pub fn from_route_and_replace_history(route: &Route, guides: &[Guide]) -> Self {
        match route {
            Route::Root => {
                match guides.first() {
                    Some(guide) => Page::Guide { guide: *guide, show_intro: true },
                    None => Page::NotFound,
                }
            },
            Route::Guide(slug) => {
                match guides.iter().find(|guide| guide.slug == slug) {
                    Some(guide) => Page::Guide { guide: *guide, show_intro: false },
                    None => Page::NotFound,
                }
            },
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
    orders.send_msg(Msg::UpdatePageTitle);

    let guides = guide::guides();

    let config: Config = local_storage()
        .get_item(STORAGE_KEY)
        .ok()
        .and_then(identity) // @TODO replace with `.flatten()` once stable
        .and_then(|serialized_config| {
            serde_json::from_str(&serialized_config).ok()
        }).unwrap_or_default();

    let model = Model {
        page: Page::from_route_and_replace_history(&url.into(), &guides),
        guide_list_visibility: Hidden,
        menu_visibility: Hidden,
        in_prerendering: is_in_prerendering(),
        guides,
        search_query: "".to_string(),
        matched_guides: vec![],
        mode: config.mode,
    };

    Init {
        model,
        url_handling: UrlHandling::None,
        mount_type: MountType::Takeover,
    }
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
    Some(Msg::RouteChanged(url.into()))
}

#[derive(Clone)]
pub enum Route {
    Root,
    Guide(String),
    Unknown,
}

impl From<Url> for Route {
    fn from(url: Url) -> Self {
        let mut path = url.path.into_iter();

        match path.next().as_ref().map(String::as_str) {
            None | Some("") => Route::Root,
            Some("guide") => path.next().map(Route::Guide).unwrap_or(Route::Unknown),
            _ => Route:: Unknown,
        }
    }
}

impl Route {
    pub fn path(&self) -> Vec<&str> {
        match self {
            Route::Root => vec![],
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
    ToggleGuideList,
    HideGuideList,
    ToggleMenu,
    HideMenu,
    SearchQueryChanged(String),
    ToggleMode,
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
                Page::Guide {..} => TITLE_SUFFIX.to_owned(),
                Page::NotFound => format!("404 - {}", TITLE_SUFFIX),
            };
            document().set_title(&title);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
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
        },
        Msg::ToggleMode => {
            model.mode.toggle();

            let config = Config { mode: model.mode };
            storage::store_data(&local_storage(), STORAGE_KEY, &config);
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

fn local_storage() -> storage::Storage {
    storage::get_storage().expect("get local storage failed")
}


// ------ ------
//     View
// ------ ------

pub fn view(model: &Model) -> impl View<Msg> {
        div![
            class![
                C.min_h_screen,
            ],
            match model.page {
                Page::Guide { guide, show_intro } => page::guide::view(&guide, model, show_intro).els(),
                Page::NotFound => page::not_found::view().els(),
            },
            page::partial::header::view(model).els(),
        ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    App::build(init, update, view)
        .routes(routes)
        .build_and_start();
}
