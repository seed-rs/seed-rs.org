use crate::{
    generated::css_classes::C,
    page::partial::{blender, image},
    Model, Msg, Page, Route,
    Visibility::Hidden,
};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> impl View<Msg> {
    nav![
        id!("header"),
        class![
            C.fixed,
            C.w_full,
            C.z_30,
            C.top_0,
            C.bg_white,
            C.shadow,
            // lg__
            C.lg__shadow_none,
        ],
        blender::view_for_header(model.mode).els(),
        // container
        div![
            class![
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
            view_container_with_border().els(),
            view_guide_list_toggle(model.page, model.in_prerendering).els(),
            view_logo().els(),
            view_menu_toggle(model.in_prerendering).els(),
            view_menu_content(model).els(),
        ]
    ]
}

// ------ view border  ------

fn view_container_with_border() -> impl View<Msg> {
    div![class![
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

fn view_guide_list_toggle(page: Page, in_prerendering: bool) -> impl View<Msg> {
    let page_is_guide = match page {
        Page::Guide {
            ..
        } => true,
        _ => false,
    };

    let toggle = button![
        id!("view_guide_list_toggle"),
        class![
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
            div![class![C.h_6, C.w_6, C.rotate], image::spinner_svg().els()]
        } else {
            span!["Guides",]
        }
    ];

    div![
        class![
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

fn view_logo() -> impl View<Msg> {
    div![
        class![
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
            class![
                C.w_24,
                C.focus__outline_none,
                // lg__
                C.lg__w_32,
            ],
            attrs! {
                At::Href => Route::Root.to_string()
            },
            image::seed_logo_svg().els(),
        ]
    ]
}

// ------ view menu  ------

fn view_menu_toggle(in_prerendering: bool) -> impl View<Msg> {
    div![
        class![
            C.relative,
            C.pr_4,
            C.flex,
            // lg__
            C.lg__hidden
        ],
        button![
            id!("menu_toggle"),
            class![
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
                div![class![C.h_6, C.w_6, C.rotate], image::spinner_svg().els()]
            } else {
                span!["Menu",]
            }
        ]
    ]
}

fn view_menu_content(model: &Model) -> impl View<Msg> {
    div![
        id!("menu_content"),
        class![
            C.w_full,
            C.relative,
            C.hidden => model.menu_visibility == Hidden,
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
        view_links().els(),
        view_github_mark().els(),
    ]
}

fn view_links() -> impl View<Msg> {
    ul![
        class![
            C.justify_end,
            C.items_center,
            // lg__
            C.lg__flex,
        ],
        view_link(
            "Rust Quickstart",
            "https://github.com/seed-rs/seed-quickstart"
        )
        .els(),
        view_link(
            "Webpack QS",
            "https://github.com/seed-rs/seed-quickstart-webpack"
        )
        .els(),
        view_link("Docs.rs", "https://docs.rs/seed/latest/seed").els(),
        view_link("Crates.io", "https://crates.io/crates/seed").els(),
        view_link("Awesome List", "https://github.com/seed-rs/awesome-seed-rs")
            .els(),
    ]
}

fn view_link(title: &str, link: &str) -> impl View<Msg> {
    li![
        class![
            C.mr_3, C.py_2, // lg__
            C.lg__py_0,
        ],
        a![
            class![
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

fn view_github_mark() -> impl View<Msg> {
    a![
        class![
            C.mt_4, C.mb_8, C.mr_8, // lg__
            C.lg__mx_3, C.lg__my_0,
        ],
        attrs! {
            At::Href => "https://github.com/seed-rs/seed",
        },
        image::github_mark_svg().els()
    ]
}
