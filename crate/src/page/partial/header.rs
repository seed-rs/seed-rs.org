// Clippy complains about `cognitive_complexity` for simple functions with macros.
#![allow(clippy::cognitive_complexity)]

use crate::{
    generated::css_classes::C,
    page::partial::{blender, image},
    Model, Msg, Page, Route,
    Visibility::Hidden,
};
use seed::{a, attrs, button, div, id, li, nav, prelude::*, span, ul, C, IF};

pub fn view(model: &Model) -> Node<Msg> {
    nav![
        id!("header"),
        C![
            C.fixed,
            C.w_full,
            C.z_30,
            C.top_0,
            C.bg_white,
            C.shadow,
            // lg__
            C.lg__shadow_none,
        ],
        blender::view_for_header(model.mode),
        // container
        div![
            C![
                C.relative,
                C.w_full,
                C.container,
                C.mx_auto,
                C.flex,
                C.flex_wrap,
                C.items_center,
                C.justify_between,
                C.mt_0,
                C.pt_2,
                C.pb_2,
            ],
            view_container_with_border(),
            view_guide_list_toggle(model.page, model.in_prerendering),
            view_logo(),
            view_menu_toggle(model.in_prerendering),
            view_menu_content(model),
        ]
    ]
}

// ------ view border  ------

fn view_container_with_border() -> Node<Msg> {
    div![C![
        C.absolute,
        C.right_0,
        C.top_0,
        C.h_full,
        C.w_11of12,
        // lg__
        C.lg__w_9of12,
        C.lg__border_b_4,
        C.lg__border_blue_500,
    ]]
}

// ------ view guide list toggle  ------

fn view_guide_list_toggle(page: Page, in_prerendering: bool) -> Node<Msg> {
    let page_is_guide = match page {
        Page::Guide {
            ..
        } => true,
        _ => false,
    };

    let toggle = button![
        id!("view_guide_list_toggle"),
        C![
            C.flex,
            C.items_center,
            C.px_3,
            C.py_2,
            C.font_bold,
            C.border_2,
            C.rounded_full,
            C.text_green_500,
            C.hover__text_green_700,
            C.border_green_500,
            C.hover__border_green_700,
            C.appearance_none,
            C.focus__outline_none,
            C.hover__underline,
        ],
        simple_ev(Ev::Click, Msg::ScrollToTop),
        simple_ev(Ev::Click, Msg::ToggleGuideList),
        if in_prerendering {
            div![C![C.h_6, C.w_6, C.rotate], image::spinner_svg()]
        } else {
            span!["Guides",]
        }
    ];

    div![
        C![
            C.relative,
            C.pl_4,
            C.flex,
            // lg__
            C.lg__hidden
        ],
        if page_is_guide {
            toggle
        } else {
            a![
                attrs! {
                    At::Href => Route::Root.to_string()
                },
                toggle,
            ]
        }
    ]
}

// ------ view logo  ------

fn view_logo() -> Node<Msg> {
    div![
        C![
            C.relative,
            C.flex,
            C.items_center,
            C.pb_px,
            // lg__
            C.lg__pb_0,
            C.lg__mt_1,
            C.lg__pl_16,
        ],
        a![
            C![
                C.w_24,
                C.focus__outline_none,
                // lg__
                C.lg__w_32,
            ],
            attrs! {
                At::Href => Route::Root.to_string()
            },
            image::seed_logo_svg(),
        ]
    ]
}

// ------ view menu  ------

fn view_menu_toggle(in_prerendering: bool) -> Node<Msg> {
    div![
        C![
            C.relative,
            C.pr_4,
            C.flex,
            // lg__
            C.lg__hidden
        ],
        button![
            id!("menu_toggle"),
            C![
                C.flex,
                C.items_center,
                C.px_3,
                C.py_2,
                C.font_bold,
                C.border_2,
                C.rounded_full,
                C.text_blue_500,
                C.hover__text_blue_700,
                C.border_blue_500,
                C.hover__border_blue_700,
                C.appearance_none,
                C.focus__outline_none,
                C.hover__underline,
            ],
            simple_ev(Ev::Click, Msg::ToggleMenu),
            if in_prerendering {
                div![C![C.h_6, C.w_6, C.rotate], image::spinner_svg()]
            } else {
                span!["Menu",]
            }
        ]
    ]
}

fn view_menu_content(model: &Model) -> Node<Msg> {
    div![
        id!("menu_content"),
        C![
            C.w_full,
            C.relative,
            IF!(model.menu_visibility == Hidden => C.hidden),
            C.mt_6,
            C.z_20,
            C.flex,
            C.flex_col,
            C.items_end,
            C.text_right,
            // lg__
            C.lg__flex,
            C.lg__flex_row,
            C.lg__content_center,
            C.lg__items_center,
            C.lg__w_auto,
            C.lg__mt_0,
        ],
        view_links(),
        view_github_mark(),
    ]
}

fn view_links() -> Node<Msg> {
    ul![
        C![
            C.justify_end,
            C.items_center,
            // lg__
            C.lg__flex,
        ],
        view_link(
            "Rust Quickstart",
            "https://github.com/seed-rs/seed-quickstart"
        ),
        view_link(
            "Webpack QS",
            "https://github.com/seed-rs/seed-quickstart-webpack"
        ),
        view_link("Docs.rs", "https://docs.rs/seed/latest/seed"),
        view_link("Crates.io", "https://crates.io/crates/seed"),
        view_link("Awesome List", "https://github.com/seed-rs/awesome-seed-rs"),
    ]
}

fn view_link(title: &str, link: &str) -> Node<Msg> {
    li![
        C![
            C.mr_3, C.py_2, // lg__
            C.lg__py_0,
        ],
        a![
            C![
                C.inline_block,
                C.py_2,
                C.px_4,
                C.text_blue_500,
                C.hover__text_blue_700,
                C.hover__underline,
                C.font_bold,
                C.focus__outline_none,
            ],
            attrs! {
                At::Href => link,
            },
            title
        ]
    ]
}

fn view_github_mark() -> Node<Msg> {
    a![
        C![
            C.mt_4, C.mb_8, C.mr_8, // lg__
            C.lg__mx_3, C.lg__my_0,
        ],
        attrs! {
            At::Href => "https://github.com/seed-rs/seed",
        },
        image::github_mark_svg()
    ]
}
