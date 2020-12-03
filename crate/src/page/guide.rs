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
            C.mx_auto,
            C.px_2,
            C.mt_16,
            // lg__
            C.lg__mt_0,
            C.container,
        ],
        guide_list::view(guide, model),
        view_content(guide, model, show_intro, &model.guide_content_el),
    ]
}

fn view_content(
    guide: &Guide,
    model: &Model,
    show_intro: bool,
    guide_content_el: &ElRef<web_sys::HtmlElement>,
) -> Node<Msg> {
    use content_control_panel::Position::{Bottom, Top};
    div![
        C![
            C.bg_white,
            // lg__
            C.lg__pt_24,
            C.content_container,
        ],
        el_ref(guide_content_el),
        IF!(model.in_prerendering => view_loading_warning()),
        intro::view(
            show_intro,
            &model.base_url,
            &model.guides,
            model.selected_seed_version
        ),
        content_control_panel::view(guide, Top, model),
        view_guide_html(guide.html),
        content_control_panel::view(guide, Bottom, model),
        view_platform_logos()
    ]
}

fn view_loading_warning() -> Node<Msg> {
    div![
        raw![
            r#"
            <style>
                @keyframes loading-warning-display {
                    from { height: 0; opacity: 0; }
                    to { height: auto; opacity: 1; }
                }
            </style>
            "#
        ],
        style! {
            St::AnimationName => "loading-warning-display",
            St::AnimationDelay => "5s",
            St::AnimationDuration => "3s",
            St::AnimationFillMode => "both",
            St::AnimationTimingFunction => "ease-in",
            St::Overflow => "hidden",
        },
        div![
            C![
                C.bg_blue_500,
                C.text_white,
                C.text_sm,
                C.p_4,
                C.mb_8,
            ],
            attrs! {
                At::Custom("role".into()) => "alert",
            },
            strong![C![C.font_bold,], "Loading..."],
            p![
                "If this message does not disappear for a long time, make sure you have a ",
                a![
                    C![C.underline,],
                    attrs! {
                        At::Href => "https://developer.mozilla.org/en-US/docs/WebAssembly#Browser_compatibility",
                    },
                    "browser with WebAssembly support"
                ],
                " and JavaScript enabled."
            ]
        ]
    ]
}

fn view_guide_html(content: &str) -> Node<Msg> {
    div![
        C![
            // it has to be "markdown-body" so it's content is styled by Github CSS
            C.markdown_body,
        ],
        set_el_keys(raw!(content))
    ]
}

fn view_netlify_logo() -> Node<Msg> {
    a![
        C! {
            C.mx_2,
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

fn view_digitalocean_logo() -> Node<Msg> {
    a![
        C! {
            C.mx_2,
            C.w_40
        },
        attrs! {
            // referral link from console
            At::Href => "https://m.do.co/c/f02c252209c1" 
        },
        img![
            C! {
                C.z_auto,
            },
            attrs! {
                At::Src => "https://opensource.nyc3.cdn.digitaloceanspaces.com/attribution/assets/PoweredByDO/DO_Powered_by_Badge_black.svg"
            }
        ],
    ]
}

fn view_platform_logos() -> Node<Msg> {
    div![
        C! {
            C.flex,
            C.mt_5,
            C.items_center,
            C.justify_center,
            C.flex_row,
        },
        view_netlify_logo(),
        view_digitalocean_logo(),
    ]
}

/// Add element keys to force reinitialization on page change.
fn set_el_keys(nodes: Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    nodes
        .into_iter()
        .map(|node| match node {
            Node::Element(mut el) => {
                if let Some(AtValue::Some(el_key_value)) =
                    el.attrs.vals.get(&At::from("data-el-key"))
                {
                    el.key = Some(el_key(el_key_value));
                }
                Node::Element(el)
            },
            _ => node,
        })
        .collect()
}
