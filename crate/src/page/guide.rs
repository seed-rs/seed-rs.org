// Clippy complains about `cognitive_complexity` for simple functions with macros.
#![allow(clippy::cognitive_complexity)]

use crate::{
    generated::css_classes::C,
    page::partial::{content_control_panel, guide_list, intro},
    Guide, Model, Msg,
};
use seed::{prelude::*, *};
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
        intro::view(show_intro),
        content_control_panel::view(guide, Top, model),
        view_guide_html(guide.html),
        content_control_panel::view(guide, Bottom, model),
        view_netlify_logo()
    ]
}

fn view_guide_html(content: &str) -> Node<Msg> {
    div![
        C![
            // it has to be "markdown-body" so it's content is styled by Github CSS
            C.markdown_body,
        ],
        raw!(content)
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
