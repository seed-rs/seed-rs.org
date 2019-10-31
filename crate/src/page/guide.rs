use crate::{generated::css_classes::C, image_src, Msg, Page, MAIL_TO_HELLWEB, Model, Guide, Route, MAIL_TO_KAVIK, previous_guide, next_guide};
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
        ],
        view_guide_list(guide, model).els(),
        view_content(guide, model).els(),
    ]
}

fn view_guide_list(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            C.text_xl,
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
    let mut guide_list: Vec<Node<Msg>> =
        model
            .guides
            .iter()
            .map(|guide| {
                let guide_is_selected = guide == selected_guide;
                let guide_is_matched = model.matched_guides.contains(guide);
                view_guide_list_item(guide, guide_is_selected, guide_is_matched).els()
            })
            .flatten()
            .collect();

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
        view_search(model).els(),
        ul![
            guide_list
        ]
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
            C.py_4,
            C.mb_6,
            // lg__
            C.lg__py_0,
        ],
        div![
            class![
                C.relative,
                C.pl_4,
                C.pr_4,
                // md__
                C.md__pr_0,
            ],
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
                    C.leading_normal,
                ],
                attrs!{
                    At::Type => "search",
                    At::Placeholder => "Search",
                    At::Value => model.search_query,
                },
                input_ev(Ev::Input, Msg::SearchQueryChanged),
            ],
            div![
                class![
                    C.absolute,
                ],
                style!{
                    St::Top => rem(0.6),
                    St::Left => rem(1.5),
                },
                svg![
                    class![
                        C.fill_current,
                        C.pointer_events_none,
                        C.text_green_800,
                        C.w_4,
                        C.h_4,
                    ],
                    attrs!{
                        At::ViewBox => "0 0 20 20",
                    },
                    path![
                        attrs!{
                            At::D => "M12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12zM12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12z",
                        }
                    ]
                ],
            ]
        ]
    ]
}

fn view_guide_list_item(guide: &Guide, active: bool, matched: bool) -> impl View<Msg> {
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
            hr![
                class![
                    C.border_t,
                    C.border_green_300,
                ]
            ]
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
                    C.text_sm,
                    C.text_green_900 => active,
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
            C.min_h_screen,
            C.p_8,
            C.mt_6,
            C.text_gray_900,
            C.leading_normal,
            C.bg_white,
            C.border_l_4,
            C.border_green_500,
            // lg__
            C.lg__w_4of5,
            C.lg__mt_0,
            C.lg__pt_24,
        ],
        view_browsing_links(guide, &model.guides, Position::Top).els(),
        view_content_markdown(guide.html).els(),
        view_browsing_links(guide, &model.guides, Position::Bottom).els(),
    ]
}

fn view_content_markdown(content: &str) -> impl View<Msg> {
    div![
        class![
            "md",
        ],
        raw!(content)
    ]
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Position {
    Top,
    Bottom
}

fn view_browsing_links(selected_guide: &Guide, guides: &[Guide], position: Position) -> impl View<Msg> {
    div![
        class![
            if position == Position::Top { C.mb_6 } else { C.mt_6 },
            C.w_full,
            C.flex,
            C.justify_between,
            C.text_green_500,
            // md__
            C.md__text_sm,
            // lg__
            C.lg__ml_auto,
        ],
        if let Some(previous_guide) = previous_guide(selected_guide, guides) {
            a![
                class![
                    C.flex,
                    C.hover__underline,
                    C.hover__text_green_700,
                    C.focus__outline_none,
                ],
                attrs! {
                    At::Href => Route::Guide(previous_guide.slug.to_owned()).to_string(),
                },
                view_previous_icon().els(),
                div![
                    class![
                        C.font_bold,
                        C.m_auto,
                        C.pb_1,
                        // md__
                        C.md__text_sm,
                    ],
                    previous_guide.menu_title,
                ],
            ]
        } else {
            empty![]
        },
        // spacer
        div![
            class![
                C.w_5,
            ]
        ],
        if let Some(next_guide) = next_guide(selected_guide, guides) {
            a![
                class![
                    C.flex,
                    C.hover__underline,
                    C.hover__text_green_700,
                    C.focus__outline_none,
                ],
                attrs! {
                    At::Href => Route::Guide(next_guide.slug.to_owned()).to_string(),
                },
                div![
                    class![
                        C.font_bold,
                        C.m_auto,
                        C.pb_1,
                        // md__
                        C.md__text_sm,
                    ],
                    next_guide.menu_title,
                ],
                view_next_icon().els(),
            ]
        } else {
            empty![]
        }
    ]
}

fn view_previous_icon() -> impl View<Msg> {
    div![
        class![
            C.h_8,
        ],
        style!{
            St::Transform => "rotate(180deg)",
        },
        next_icon_svg().els()
    ]
}

fn view_next_icon() -> impl View<Msg> {
    div![
        class![
            C.h_8,
        ],
        next_icon_svg().els()
    ]
}

fn next_icon_svg() -> impl View<Msg> {
    raw![
        r#"
            <svg width="100%" height="100%" viewBox="0 0 27 44" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;">
                <g transform="matrix(1,0,0,1,-8363.26,-3858.28)">
                    <g transform="matrix(0.739583,0,0,3.93945,4533.22,0)">
                        <g transform="matrix(0.580448,0,0,0.242691,2561.61,518.56)">
                            <path d="M4508.65,1921.38L4554.22,1921.38L4524.11,1898.85L4539.16,1898.85L4569.27,1921.38L4539.16,1943.91L4524.11,1943.91L4554.22,1921.38L4508.65,1921.38Z" style="fill:currentColor;"/>
                        </g>
                    </g>
                </g>
            </svg>
        "#
    ]
}