use crate::{generated::css_classes::C, Msg, Model, Guide, Route, previous_guide, next_guide, Visibility};
use seed::{prelude::*, *};
use crate::Visibility::{Hidden, Visible};

pub fn view(guide: &Guide, model: &Model, show_intro: bool) -> impl View<Msg> {
    div![
        class![
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
        view_guide_list(guide, model).els(),
        view_content(guide, model, show_intro).els(),
    ]
}

fn view_guide_list(guide: &Guide, model: &Model) -> impl View<Msg> {
    div![
        class![
            C.w_full,
            // lg__
            C.lg__w_1of5,
            C.lg__px_6,
        ],
        view_guide_list_toggle(guide, model.guide_list_visibility).els(),
        view_guide_list_items(guide, model).els(),
    ]
}

fn view_guide_list_toggle(selected_guide: &Guide, guide_list_visibility: Visibility) -> impl View<Msg> {
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
            svg![
                class![
                    C.mt_1,
                    C.fill_current,
                    C.h_5,
                    C.float_right,
                ],
                style!{
                    St::Transform => if guide_list_visibility == Visible { "rotate(180deg)" } else { "none" }
                },
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
        ul![
            model
                .guides
                .iter()
                .map(|guide| {
                    let guide_is_selected = guide == selected_guide;
                    let guide_is_matched = model.matched_guides.contains(guide);
                    view_guide_list_item(guide, guide_is_selected, guide_is_matched).els()
                })
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
            C.pt_4,
            C.mb_6,
            // lg__
            C.lg__pt_0,
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
                    C.text_green_900 => active,
                    C.font_bold => active,
                ],
                guide.menu_title,
            ]
        ]
    ]
}

fn view_content(guide: &Guide, model: &Model, show_intro: bool) -> impl View<Msg> {
    div![
        class![
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
        if show_intro { view_intro().els() } else { vec![] },
        view_browsing_links(guide, &model.guides, Position::Top).els(),
        view_content_markdown(guide.html).els(),
        view_browsing_links(guide, &model.guides, Position::Bottom).els(),
    ]
}

fn view_intro() -> impl View<Msg> {
    div![
        div![
            class![
                C.mb_8,
                // sm__
                C.sm__my_12,
                C.sm__flex,
                C.sm__justify_center,
                C.sm__items_center,
            ],
            view_logo().els(),
            h2![
                class![
                    C.font_semibold,
                    C.text_right,
                    C.mt_2,
                    // sm__
                    C.sm__text_xl,
                    C.sm__mt_0,
                    C.sm__ml_12,
                ],
                "Rust framework for creating",
                br![],
                "fast and reliable web apps",
            ],
        ],
        view_testimonials().els(),
    ]
}

fn view_logo() -> impl View<Msg> {
    div![
        class![
            C.flex,
        ],
        a![
            class![
                C.w_48,
                C.focus__outline_none,
                // lg__
                C.lg__w_64,
            ],
            attrs!{
                At::Href => Route::Root.to_string()
            },
            seed_logo_svg().els(),
        ]
    ]
}

fn seed_logo_svg() -> impl View<Msg> {
    raw![
        r#"
            <svg width="100%" height="100%" viewBox="0 0 946 404" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:1.41421;">
                <g transform="matrix(0.999934,0,0,0.999727,-93.1886,-208.737)">
                    <g transform="matrix(1,0,0,1,-113.496,-5)">
                        <path d="M1152.75,244.304L1152.75,509.522C1152.75,566.58 1106.43,612.904 1049.37,612.904C992.313,612.904 945.989,566.58 945.989,509.522C945.989,452.464 992.313,406.14 1049.37,406.14C1071.9,406.14 1092.76,413.364 1109.75,425.62L1109.75,244.304L1152.75,244.304ZM1049.37,449.139C1082.7,449.139 1109.75,476.196 1109.75,509.522C1109.75,542.848 1082.7,569.905 1049.37,569.905C1016.05,569.905 988.988,542.848 988.988,509.522C988.988,476.196 1016.05,449.139 1049.37,449.139Z"/>
                    </g>
                    <g transform="matrix(1,0,0,1,164.999,-5)">
                        <path d="M637.757,531.022C627.852,577.776 586.305,612.904 536.618,612.904C479.559,612.904 433.235,566.58 433.235,509.522C433.235,452.464 479.559,406.14 536.618,406.14C586.305,406.14 627.852,441.268 637.757,488.022L593.054,488.022C584.383,465.298 562.372,449.139 536.618,449.139C510.863,449.139 488.853,465.298 480.181,488.022L536.618,488.022L553.209,509.522L536.618,531.022L480.181,531.022C488.853,553.746 510.863,569.905 536.618,569.905C562.372,569.905 584.383,553.746 593.054,531.022L637.757,531.022Z"/>
                    </g>
                    <g transform="matrix(1,0,0,1,-69.259,-5)">
                        <path d="M637.757,531.022C627.852,577.776 586.305,612.904 536.618,612.904C479.559,612.904 433.235,566.58 433.235,509.522C433.235,452.464 479.559,406.14 536.618,406.14C586.305,406.14 627.852,441.268 637.757,488.022L593.054,488.022C584.383,465.298 562.372,449.139 536.618,449.139C510.863,449.139 488.853,465.298 480.181,488.022L536.618,488.022L553.209,509.522L536.618,531.022L480.181,531.022C488.853,553.746 510.863,569.905 536.618,569.905C562.372,569.905 584.383,553.746 593.054,531.022L637.757,531.022Z"/>
                    </g>
                    <path d="M318.203,329.008L274.538,329.008C274.226,310.375 265.491,293.777 251.978,282.861L223.098,311.629C218.025,316.682 209.822,316.682 204.749,311.629L175.869,282.861C162.108,293.978 153.301,310.989 153.301,330.04C153.301,363.499 180.465,390.663 213.924,390.663L213.924,390.665L225.479,391.478C230.529,392.04 235.379,392.793 240.291,394.093L257.723,380.664L261.558,402.332C266.064,404.681 270.39,407.359 274.5,410.346L295.607,404.121L291.355,425.711C294.709,429.529 297.775,433.589 300.529,437.859L322.459,439.678L310.695,458.275C312.443,463.046 313.835,467.94 314.861,472.917L334.653,482.535L316.965,495.627C316.872,500.707 316.402,505.773 315.561,510.784L330.541,526.903L309.319,532.721C307.397,537.425 305.129,541.979 302.534,546.348L310.68,566.79L288.789,564.549C285.298,568.24 281.538,571.667 277.54,574.804L277.752,596.808L258.149,586.811C253.559,588.991 248.815,590.829 243.954,592.31L236.203,612.904L221.535,596.5C216.468,596.876 211.38,596.876 206.313,596.5L191.645,612.904L183.893,592.31C179.033,590.829 174.288,588.991 169.699,586.811L150.096,596.808L150.308,574.804C146.31,571.667 142.55,568.24 139.058,564.549L117.167,566.79L125.313,546.348C122.719,541.979 120.451,537.425 118.528,532.721L97.306,526.903L112.287,510.784C111.445,505.773 110.976,500.707 110.882,495.627L110.918,492.69L153.31,492.69C153.304,493.034 153.301,493.378 153.301,493.723C153.301,511.98 161.389,528.364 174.175,539.482L204.728,509.048C209.813,503.983 218.035,503.983 223.119,509.048L253.672,539.482C266.458,528.364 274.547,511.98 274.547,493.723C274.547,460.264 247.383,433.1 213.924,433.099C211.384,433.099 208.846,433.006 206.313,432.818L191.645,449.222L183.893,428.627C179.033,427.147 174.288,425.309 169.699,423.128L150.096,433.126L150.308,411.122C146.31,407.985 142.55,404.557 139.058,400.866L117.167,403.107L125.313,382.665C122.719,378.297 120.451,373.742 118.528,369.039L97.306,363.221L112.287,347.102C111.445,342.091 110.976,337.025 110.882,331.945L93.195,318.853L112.987,309.234C114.012,304.258 115.405,299.364 117.152,294.593L105.389,275.996L127.319,274.177C130.073,269.907 133.139,265.846 136.492,262.029L132.241,240.438L153.347,246.664C157.458,243.677 161.784,240.998 166.289,238.65L170.125,216.981L187.557,230.411C192.469,229.111 197.47,228.176 202.52,227.614L213.924,208.794L225.328,227.614C230.377,228.176 235.379,229.111 240.291,230.411L257.723,216.981L261.558,238.65C266.064,240.998 270.39,243.677 274.5,246.664L295.607,240.438L291.355,262.029C294.709,265.846 297.775,269.907 300.529,274.177L322.459,275.996L310.695,294.593C312.443,299.364 313.835,304.258 314.861,309.234L318.203,329.008ZM213.924,562.544C205.477,562.544 198.619,555.686 198.619,547.239C198.619,538.792 205.477,531.934 213.924,531.934C222.371,531.934 229.229,538.792 229.229,547.239C229.229,555.686 222.371,562.544 213.924,562.544ZM213.924,258.112C222.371,258.112 229.229,264.97 229.229,273.417C229.229,281.864 222.371,288.722 213.924,288.722C205.477,288.722 198.619,281.864 198.619,273.417C198.619,264.97 205.477,258.112 213.924,258.112Z"/>
                </g>
            </svg>
        "#
    ]
}


struct Testimonial {
    quote: &'static str,
    url: &'static str,
    author_image_url: &'static str,
}

fn view_testimonials() -> impl View<Msg> {
    let testimonials = vec![
        Testimonial {
            quote: "Awesome, awesome framework!",
            url: "https://github.com/David-OConnor/seed/issues/193#issue-479188076",
            author_image_url: "https://avatars.githubusercontent.com/u/16214",
        },
        Testimonial {
            quote: "Seed rocks",
            url: "https://github.com/MartinKavik/seed-rs-realworld/issues/1#issuecomment-525413644",
            author_image_url: "https://avatars.githubusercontent.com/u/48671239",
        },
        Testimonial {
            quote: "I’m super stoked about this framework.",
            url: "https://github.com/David-OConnor/seed/issues/11#issuecomment-457477672",
            author_image_url: "https://avatars.githubusercontent.com/u/2380740",
        },
        Testimonial {
            quote: "cool, Elm but in Rust!",
            url: "https://github.com/David-OConnor/seed/issues/52#issue-412081499",
            author_image_url: "https://avatars.githubusercontent.com/u/38404589",
        },
        Testimonial {
            quote: "this is a pretty cool Rust web framework!",
            url: "https://github.com/David-OConnor/seed/issues/16#issue-395014777",
            author_image_url: "https://avatars.githubusercontent.com/u/139499",
        },
        Testimonial {
            quote: "I'm very new to Rust and Seed is the only frontend framework I find accessible.",
            url: "https://github.com/David-OConnor/seed/issues/31#issue-403427680",
            author_image_url: "https://avatars.githubusercontent.com/u/45914742",
        },
        Testimonial {
            quote: "this framework looks very promising and the getting-started experience was very smooth thanks to the excellent documentation!",
            url: "https://github.com/David-OConnor/seed/issues/5#issue-392002515",
            author_image_url: "https://avatars.githubusercontent.com/u/16122",
        },
        Testimonial {
            quote: "I love it",
            url: "https://github.com/David-OConnor/seed/issues/192#issuecomment-518190059",
            author_image_url: "https://avatars.githubusercontent.com/u/16864501",
        },
        Testimonial {
            quote: "It composes really well and feels very Rusty :)",
            url: "https://github.com/David-OConnor/seed/issues/193#issuecomment-536255691",
            author_image_url: "https://avatars.githubusercontent.com/u/7584365",
        },
        Testimonial {
            quote: "I like it and hope it become the wide-using wasm webapp dev framework.",
            url: "https://github.com/David-OConnor/seed/issues/111#issue-443462538",
            author_image_url: "https://avatars.githubusercontent.com/u/163317",
        }
    ];

    let (testimonials_1, testimonials_2) = testimonials.split_at(testimonials.len() / 2);
    div![
        class![
            C.mb_10,
            // md__
            C.md__mb_0,
            C.md__flex,
        ],
        ul![
            testimonials_1.iter().map(view_testimonial)
        ],
        ul![
            class![
                // md__
                C.md__ml_4,
            ],
            testimonials_2.iter().map(view_testimonial)
        ]
    ]
}


fn view_testimonial(testimonial: &Testimonial) -> Node<Msg> {
    li![
        a![
            class![
                C.flex,
                C.my_5,
                C.items_center,
                C.hover__underline,
                C.hover__text_green_900,
                C.text_green_700,
            ],
            attrs!{
                At::Href => testimonial.url,
            },
            img![
                class![
                    C.object_contain,
                    C.flex_shrink_0,
                    C.rounded_full,
                ],
                attrs!{
                    At::Src => format!("{}{}", testimonial.author_image_url, "?v=4&s=48"),
                    At::Height => 48,
                    At::Width => 48,
                },
            ],
            div![
                class![
                    C.mx_2,
                ],
                testimonial.quote,
            ],
        ],
    ]
}

fn view_content_markdown(content: &str) -> impl View<Msg> {
    div![
        class![
            // it has to be "markdown-body" so it's content is styled by Github CSS
            "markdown-body",
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
            if position == Position::Top { C.mb_8 } else { C.mt_8 },
            C.w_full,
            C.flex,
            C.justify_between,
            C.text_green_500,
            C.text_sm,
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
                        C.hidden,
                        // sm__,
                        C.sm__block,
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
                C.w_5
            ],
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
                        C.hidden,
                        // sm__,
                        C.sm__block,
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