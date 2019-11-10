use crate::{
    generated::css_classes::C, page::partial::image, Guide, Model, Msg, Route,
    Visibility::Hidden,
};
use seed::{prelude::*, *};

pub fn view(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            // lg__
            C.lg__w_1of5,
            C.lg__px_6,
        ],
        view_guide_list_toggle(guide, model.in_prerendering).els(),
        view_guide_list_content(guide, model).els(),
    ]
}

// ------ view guide list toggle  ------

fn view_guide_list_toggle(
    selected_guide: &Guide,
    in_prerendering: bool,
) -> impl View<Msg> {
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
                C.pl_8,
                C.pr_8,
                C.py_3,
                C.bg_white,
                C.rounded,
                C.text_white,
                C.hover__text_green_200,
                C.appearance_none,
                C.focus__outline_none,
                C.font_bold,
                C.rounded_full,
                C.bg_green_500,
            ],
            simple_ev(Ev::Click, Msg::ToggleGuideList),
            selected_guide.menu_title,
            if in_prerendering {
                vec![div![
                    class![C.h_6, C.w_6, C.rotate],
                    image::spinner_svg().els()
                ]]
            } else {
                view_hamburger().els()
            }
        ]
    ]
}

fn view_hamburger() -> impl View<Msg> {
    div![
        class![C.text_2xl, C.leading_none,],
        // TRIGRAM FOR HEAVEN - https://www.fileformat.info/info/unicode/char/2630/index.htm
        "\u{2630}"
    ]
}

// ------ view guide list content ------

fn view_guide_list_content(
    selected_guide: &Guide,
    model: &Model,
) -> impl View<Msg> {
    div![
        id!("menu_items"),
        class![
            C.w_10of12,
            C.inset_0,
            C.m_auto,
            C.hidden => model.guide_list_visibility == Hidden,
            C.overflow_x_hidden,
            C.overflow_y_auto,
            C.mt_0,
            C.border_4,
            C.border_t_0,
            C.border_green_500,
            C.bg_white,
            C.z_20,
            // lg__
            C.lg__w_full,
            C.lg__sticky,
            C.lg__overflow_y_hidden,
            C.lg__border_transparent,
            C.lg__bg_transparent,
            C.lg__block,
        ],
        style! {
            St::Top => em(7),
        },
        view_search(model).els(),
        ul![model.guides.iter().map(|guide| {
            let guide_is_selected = guide == selected_guide;
            let guide_is_matched = model.matched_guides.contains(guide);
            view_guide_list_item(guide, guide_is_selected, guide_is_matched)
                .els()
        })]
    ]
}

fn view_search(model: &Model) -> impl View<Msg> {
    div![
        class![
            C.flex_1,
            C.w_full,
            C.mx_auto,
            C.max_w_sm,
            C.content_center,
            C.pt_4,
            C.mb_6,
            // lg__
            C.lg__pt_0,
        ],
        div![
            class![
                C.relative, C.pl_4, C.pr_4, // md__
                C.md__pr_0,
            ],
            // search icon
            div![
                class![C.absolute,],
                style! {
                    St::Top => rem(0.6),
                    St::Left => rem(1.5),
                },
                image::search_icon_svg().els()
            ],
            // search input
            input![
                class![
                    C.w_full,
                    C.bg_green_100,
                    C.text_sm,
                    C.text_green_800,
                    C.placeholder_green_800,
                    C.border_b_4,
                    C.border_green_500,
                    C.focus__outline_none,
                    C.pt_2,
                    C.pb_2,
                    C.px_2,
                    C.pl_8,
                    C.appearance_none,
                ],
                attrs! {
                    At::Type => "search",
                    At::Placeholder => "Search",
                    At::Value => model.search_query,
                },
                input_ev(Ev::Input, Msg::SearchQueryChanged),
            ],
        ]
    ]
}

fn view_guide_list_item(
    guide: &Guide,
    active: bool,
    matched: bool,
) -> impl View<Msg> {
    li![
        class![
            C.hover__bg_green_100 => !matched,
            C.bg_green_200 => matched,
            // md__
            C.md__my_0,
            // lg__
            C.lg__hover__bg_transparent => !matched,
        ],
        if guide.prepend_menu_divider {
            hr![class![C.border_t, C.border_green_300,]]
        } else {
            empty![]
        },
        a![
            class![
                C.block,
                C.py_2,
                C.pl_4,
                C.align_middle,
                C.text_green_800,
                C.hover__text_green_500,
                C.border_l_4,
                C.border_transparent,
                C.focus__outline_none,
                // lg__
                C.lg__border_green_500 => active,
                if active { C.lg__hover__border_green_500 } else { C.lg__hover__border_green_400 },
            ],
            attrs! {
                At::Href => Route::Guide(guide.slug.to_owned()).to_string(),
            },
            simple_ev(Ev::Click, Msg::HideGuideList),
            span![
                class![
                    C.block,
                    C.pb_1,
                    C.text_green_900 => active,
                    C.font_bold => active,
                ],
                guide.menu_title,
            ]
        ]
    ]
}
