use crate::{generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    footer![
        class![
            C.bg_white,
            C.border_t,
            C.border_gray_400,
            C.shadow,
        ],
        div![
            class![
                C.container,
                C.mx_auto,
                C.flex,
                C.py_8,
            ],
            div![
                class![
                    C.w_full,
                    C.mx_auto,
                    C.flex,
                    C.flex_wrap,
                ],
                view_left_col().els(),
                view_right_col().els(),
            ]
        ]
    ]
}

fn view_left_col() -> impl View<Msg> {
    div![
        class![
            C.flex,
            C.w_full,
            // lg__
            C.lg__w_1of2,
        ],
        div![
            class![
                C.px_8,
            ],
            h3![
                class![
                    C.font_bold,
                    C.text_gray_900,
                    "About",
                ]
            ],
            p![
                class![
                    C.py_4,
                    C.text_gray_600,
                    C.text_sm,
                ],
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas vel mi ut felis tempus commodo nec id erat. Suspendisse consectetur dapibus velit ut lacinia.",
            ]
        ]
    ]
}

fn view_right_col() -> impl View<Msg> {
    div![
        class![
            C.flex,
            C.w_full,
            // lg__
            C.lg__w_1of2,
            C.lg__justify_end,
            C.lg__text_right,
        ],
        div![
            class![
                C.px_8,
            ],
            h3![
                class![
                    C.font_bold,
                    C.text_gray_900,
                ],
                "Social",
            ],
            ul![
                class![
                    C.items_center,
                    C.text_sm,
                    C.pt_3,
                ],
                li![
                    a![
                        class![
                            C.inline_block,
                            C.text_gray_600,
                            C.no_underline,
                            C.hover__text_gray_900,
                            C.hover__underline,
                            C.py_1,
                        ],
                        attrs!{
                            At::Href => "",
                        },
                        "Add social links",
                    ]
                ]
            ]
        ]
    ]
}