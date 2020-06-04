// Clippy complains about `cognitive_complexity` for simple functions with macros.
#![allow(clippy::cognitive_complexity)]

use crate::{
    generated::css_classes::C,
    page::partial::{content_control_panel, guide_list, image, intro},
    Guide, Model, Msg,
};
use seed::{prelude::*, *};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub fn view(guide: &Guide, model: &Model, show_intro: bool) -> Node<Msg> {
    div![
        C![
            C.container,
            C.w_full,
            C.flex,
            C.flex_wrap,
            C.mx_auto,
            C.px_2,
            C.mt_16,
            // lg__
            C.lg__mt_0,
        ],
        guide_list::view(guide, model),
        view_content(guide, model, show_intro),
    ]
}

fn view_content(guide: &Guide, model: &Model, show_intro: bool) -> Node<Msg> {
    use content_control_panel::Position::{Bottom, Top};
    div![
        C![
            C.w_full,
            C.min_h_screen,
            C.p_8,
            C.bg_white,
            // lg__
            C.lg__w_4of5,
            C.lg__pt_24,
            C.lg__border_l_4,
            C.lg__border_green_500,
        ],
        view_prerendered_warning(model.in_prerendering),
        intro::view(
            show_intro,
            &model.base_url,
            &model.guides,
            model.selected_seed_version
        ),
        content_control_panel::view(guide, Top, model),
        view_guide_html(guide.html),
        content_control_panel::view(guide, Bottom, model),
        view_netlify_logo()
    ]
}

fn view_prerendered_warning(in_prerendering: bool) -> Node<Msg> {
    div![
        C![
            C.flex,
            C.items_center,
            C.bg_blue_500,
            C.text_white,
            C.text_sm,
            C.font_bold,
            C.px_4,
            C.py_3,
            C.mb_8,
            C.sm__mb_8,
            IF!(!in_prerendering => C.hidden),
        ],
        attrs! {
            At::Custom("role".into()) => "alert"
        },
        div![C![C.py_1,], image::info_icon_svg(),],
        p![
            "This is a pre rendered page. Please make sure your have a ",
            a![
                C![C.underline,],
                attrs! {
                    At::Href => "https://developer.mozilla.org/en-US/docs/WebAssembly#Browser_compatibility",
                },
                "browser with WebAssembly support"
            ],
            " and JavaScript enabled to get all page functionality."
        ]
    ]
}

fn view_guide_html(content: &str) -> Node<Msg> {
    div![
        C![
            // it has to be "markdown-body" so it's content is styled by Github CSS
            C.markdown_body,
        ],
        set_keys_to_code_blocks(raw!(content))
    ]
}

fn view_netlify_logo() -> Node<Msg> {
    a![
        C! {
            C.flex,
            C.mt_5,
            C.justify_center,
        },
        attrs! {
            At::Href => "https://www.netlify.com",
        },
        img![
            C! {
                C.z_auto,
            },
            attrs! {
                At::Src => "https://www.netlify.com/img/global/badges/netlify-light.svg"
            }
        ],
    ]
}

/// Add element key to each code block to force reinitialization if code changed.
fn set_keys_to_code_blocks(nodes: Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    nodes
        .into_iter()
        .map(|node| match node {
            Node::Element(el) => {
                if el.is_custom() && el.tag.as_str() == "code-block" {
                    let mut hasher = DefaultHasher::new();
                    el.get_text().hash(&mut hasher);
                    custom![el.tag, el_key(&hasher.finish()), el.children]
                } else {
                    Node::Element(el)
                }
            },
            _ => node,
        })
        .collect()
}
