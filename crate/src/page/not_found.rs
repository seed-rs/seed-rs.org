// Clippy complains about `cognitive_complexity` for simple functions with macros.
#![allow(clippy::cognitive_complexity)]

use crate::{generated::css_classes::C, Msg, Route};
use seed::{a, attrs, div, prelude::*, C};

pub fn view() -> Node<Msg> {
    div![
        C![C.mt_32, C.flex, C.justify_center,],
        div![
            C![
                C.text_2xl,
                // sm__
                C.sm__text_4xl,
                // lg__
                C.lg__text_6xl,
            ],
            div![C![C.font_bold,], "404",],
            div![C![C.my_12,], "Page not found"],
            a![
                C![
                    C.block,
                    C.text_right,
                    C.text_green_500,
                    C.hover__underline,
                    C.hover__text_green_700,
                ],
                attrs! {
                    At::Href => Route::Root.to_string()
                },
                "Home"
            ],
        ],
    ]
}
