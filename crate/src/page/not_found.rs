use crate::{generated::css_classes::C, Msg, Route};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div![
        class![
            C.mt_32,
            C.flex,
            C.justify_center,
        ],
        div![
            class![
                C.text_2xl,
                // sm__
                C.sm__text_4xl,
                // lg__
                C.lg__text_6xl,
            ],
            div![
                class![
                    C.font_bold,
                ],
                "404",
            ],
            div![
                class![
                    C.my_12,
                ],
                "Page not found"
            ],
            a![
                class![
                    C.block,
                    C.text_right,
                    C.text_green_500,
                    C.hover__underline,
                    C.hover__text_green_700,
                ],
                attrs!{
                    At::Href => Route::Root.to_string()
                },
                "Home"
            ],
        ],
    ]
}