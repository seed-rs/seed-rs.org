use crate::{
    generated::css_classes::C, image_src, Msg, Page, MAIL_TO_HELLWEB, Model, Guide, Route,
    MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};
use crate::Visibility::Hidden;

pub fn view(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.container,
            C.w_full,
            C.flex,
            C.flex_wrap,
            C.mx_auto,
            C.px_2,
            C.pt_8,
            C.mt_16,
            // lg__
            C.lg__pt_16,
        ],
        view_guide_list(guide, model).els(),
        view_content(guide, model).els(),
        view_back_link().els(),
    ]
}

fn view_guide_list(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.text_xl,
            C.text_gray_800,
            C.leading_normal,
            // lg__
            C.lg__w_1of5,
            C.lg__px_6,
        ],
        view_guide_list_toggle(guide).els(),
        view_guide_list_items(guide, model).els(),
    ]
}

fn view_guide_list_toggle(selected_guide: &Guide) -> impl View<Msg> {
    div![
        class![
            C.sticky,
            C.inset_0,
            // lg__
            C.lg__hidden,
        ],
        button![
            id!("guide_list_toggle"),
            class![
                C.flex,
                C.w_full,
                C.justify_between,
                C.px_4,
                C.py_3,
                C.bg_white,
                C.border,
                C.rounded,
                C.border_gray_600,
                C.hover__border_purple_500,
                C.appearance_none,
                C.focus__outline_none,
                C.text_sm,
                C.font_bold,
                // lg__
                C.lg__bg_transparent,
            ],
            simple_ev(Ev::Click, Msg::ToggleGuideList),
            selected_guide.menu_title,
            svg![
                class![
                    C.fill_current,
                    C.h_5,
                    C.float_right,
                ],
                attrs!{
                    At::ViewBox => "0 0 20 20",
                },
                path![
                    attrs!{
                        At::D => "M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z",
                    }
                ]
            ],
        ]
    ]
}

fn view_guide_list_items(selected_guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        id!("menu_items"),
        class![
            C.w_full,
            C.inset_0,
            C.hidden => model.guide_list_visibility == Hidden,
            C.overflow_x_hidden,
            C.overflow_y_auto,
            C.mt_0,
            C.border,
            C.border_gray_400,
            C.bg_white,
            C.shadow,
            C.z_20,
            // lg__
            C.lg__sticky,
            C.lg__overflow_y_hidden,
            C.lg__border_transparent,
            C.lg__shadow_none,
            C.lg__bg_transparent,
            C.lg__block,
        ],
        style! {
            St::Top => em(7),
        },
        ul![
            model.guides.iter().map(|guide| view_guide_list_item(guide, guide == selected_guide).els())
        ]
    ]
}

fn view_guide_list_item(guide: &Guide, active: bool) -> impl View<Msg> {
    li![
        class![
            C.hover__bg_purple_100,
            // md__
            C.md__my_0,
            // lg__
            C.lg__hover__bg_transparent,
        ],
        a![
            class![
                C.block,
                C.py_2,
                C.pl_4,
                C.align_middle,
                C.text_gray_700,
                C.hover__text_purple_500,
                C.border_l_4,
                C.border_transparent,
                C.focus__outline_none,
                // lg__
                C.lg__border_purple_500 => active,
                if active { C.lg__hover__border_purple_500 } else { C.lg__hover__border_purple_400 },
            ],
            attrs! {
                At::Href => Route::Guide(guide.slug.to_owned()).to_string(),
            },
            simple_ev(Ev::Click, Msg::HideGuideList),
            span![
                class![
                    C.block,
                    C.pb_1,
                    C.text_sm,
                    C.text_gray_900 => active,
                    C.font_bold => active,
                    // md__
                    C.md__pb_0,
                ],
                guide.menu_title,
            ]
        ]
    ]
}

fn view_content(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.p_8,
            C.mt_6,
            C.text_gray_900,
            C.leading_normal,
            C.bg_white,
            C.border,
            C.border_gray_400,
            C.rounded,
            // lg__
            C.lg__w_4of5,
            C.lg__mt_0,
        ],
        view_content_top_back_link().els(),
        view_content_markdown(guide.html).els(),
    ]
}

fn view_content_top_back_link() -> impl View<Msg> {
    div![
        class![
            C.font_sans,
        ],
        span![
            class![
                C.text_base,
                C.text_purple_500,
                C.font_bold,
            ],
            "« ",
            a![
                class![
                    C.text_base,
                    C.text_purple_500,
                    C.font_bold,
                    C.hover__underline,
                    // md__
                    C.md__text_sm,
                ],
                attrs!{
                    At::Href => "",
                },
                "Back link",
            ]
        ],
    ]
}

fn view_content_markdown(content: &str) -> impl View<Msg> {
    div![
        class![
            "markdown"
        ],
        raw!(content)
    ]
}

fn view_back_link() -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.text_gray_500,
            C.px_4,
            C.py_6,
            // md__
            C.md__text_sm,
            // lg__
            C.lg__w_4of5,
            C.lg__ml_auto,
            C.text_base,
        ],
        span![
            class![
                C.text_base,
                C.text_purple_500,
                C.font_bold,
            ],
            "< ",
            a![
                class![
                    C.text_base,
                    C.text_purple_500,
                    C.font_bold,
                    C.hover__underline,
                    // md__
                    C.md__text_sm,
                ],
                attrs!{
                    At::Href => "",
                },
                "Back to Help"
            ],
        ]
    ]
}