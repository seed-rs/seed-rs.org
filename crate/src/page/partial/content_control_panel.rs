// Clippy complains about `cognitive_complexity` for simple functions with macros.
#![allow(clippy::cognitive_complexity)]

use crate::{
    generated::css_classes::C, page::partial::image, Guide, Mode, Model, Msg,
    Urls,
};
use seed::{a, attrs, div, empty, prelude::*, span, style, C, IF};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Top,
    Bottom,
}

pub fn view(
    selected_guide: &Guide,
    position: Position,
    model: &Model,
) -> Node<Msg> {
    div![
        C![
            if position == Position::Top {
                C.mb_8
            } else {
                C.mt_8
            },
            C.w_full,
            C.flex,
            C.justify_between,
            C.text_green_500,
            C.text_sm,
            // lg__
            C.lg__ml_auto,
        ],
        // previous guide
        previous_guide(selected_guide, &model.guides).map_or_else(
            view_empty_column,
            |previous_guide| {
                view_previous_guide_link(previous_guide, &model.base_url)
            }
        ),
        // mode toggle or edit this page button
        if position == Position::Top {
            view_mode_toggle(model.in_prerendering, model.mode)
        } else {
            view_edit_this_page(selected_guide.edit_url)
        },
        // next guide
        next_guide(selected_guide, &model.guides).map_or_else(
            view_empty_column,
            |next_guide| view_next_guide_link(next_guide, &model.base_url)
        ),
    ]
}

// ------ view empty column ------

fn view_empty_column() -> Node<Msg> {
    div![C![C.flex_1,]]
}

// ------ view mode toggle ------

fn view_mode_toggle(in_prerendering: bool, mode: Mode) -> Node<Msg> {
    div![
        C![C.flex_1, C.flex, C.justify_center,],
        div![
            C![
                C.flex,
                C.items_center,
                C.px_3,
                C.text_gray_500,
                C.border,
                C.border_gray_400,
                C.rounded_full,
                IF!(!in_prerendering => C.cursor_pointer),
                IF!(!in_prerendering => C.hover__underline),
                IF!(!in_prerendering => C.hover__text_gray_700),
                IF!(!in_prerendering => C.hover__border_gray_600),
            ],
            ev(Ev::Click, |_| Msg::ToggleMode),
            span![
                C![C.whitespace_no_wrap, C.flex, C.items_center,],
                if in_prerendering {
                    div![
                        C![C.mr_1, C.h_4, C.w_4, C.rotate,],
                        image::spinner_svg()
                    ]
                } else {
                    empty![]
                },
                span![format!(
                    "{} mode",
                    match mode {
                        Mode::Light => "Dark",
                        Mode::Dark => "Light",
                    }
                ),]
            ]
        ]
    ]
}

// ------ view edit this page & feedback ------

fn view_edit_this_page(edit_url: &str) -> Node<Msg> {
    div![
        C![C.flex_1, C.flex, C.justify_center,],
        a![
            C![
                C.flex,
                C.items_center,
                C.text_blue_500,
                C.whitespace_no_wrap,
                C.hover__underline,
                C.hover__text_blue_700,
            ],
            attrs! {
                At::Href => edit_url,
            },
            span!["Edit this page",]
        ],
        span![C![C.flex, C.mx_1, C.items_center,], "|"],
        a![
            C![
                C.flex,
                C.items_center,
                C.text_blue_500,
                C.whitespace_no_wrap,
                C.hover__underline,
                C.hover__text_blue_700,
            ],
            attrs! {
                At::Href => "https://github.com/seed-rs/seed/issues/303",
            },
            span!["Feedback",]
        ]
    ]
}

// ------ view previous & next guide link ------

fn view_previous_guide_link(
    previous_guide: &Guide,
    base_url: &Url,
) -> Node<Msg> {
    div![
        C![C.flex_1, C.flex, C.justify_start,],
        a![
            C![
                C.flex,
                C.hover__underline,
                C.hover__text_green_700,
                C.focus__outline_none,
            ],
            attrs! {
                At::Href => Urls::new(base_url).guide(previous_guide),
            },
            view_previous_icon(),
            div![
                C![
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
    ]
}

fn view_next_guide_link(next_guide: &Guide, base_url: &Url) -> Node<Msg> {
    div![
        C![C.flex_1, C.flex, C.justify_end,],
        a![
            C![
                C.flex,
                C.hover__underline,
                C.hover__text_green_700,
                C.focus__outline_none,
            ],
            attrs! {
                At::Href => Urls::new(base_url).guide(next_guide),
            },
            div![
                C![
                    C.font_bold,
                    C.m_auto,
                    C.pb_1,
                    C.hidden,
                    // sm__,
                    C.sm__block,
                ],
                next_guide.menu_title,
            ],
            view_next_icon(),
        ]
    ]
}

// ------ view previous & next icon ------

fn view_previous_icon() -> Node<Msg> {
    div![
        C![C.h_8, C.w_8,],
        style! {
            St::Transform => "rotate(180deg)",
        },
        image::next_icon_svg()
    ]
}

fn view_next_icon() -> Node<Msg> {
    div![C![C.h_8, C.w_8], image::next_icon_svg()]
}

// ------ get previous & next guide ------

pub fn previous_guide<'a>(
    selected_guide: &Guide,
    guides: &'a [Guide],
) -> Option<&'a Guide> {
    let guides = guides
        .iter()
        .filter(|guide| guide.seed_version == selected_guide.seed_version)
        .collect::<Vec<_>>();

    let selected_guide_index =
        guides.iter().position(|guide| *guide == selected_guide)?;

    selected_guide_index
        .checked_sub(1)
        .and_then(|index| guides.get(index).copied())
}

pub fn next_guide<'a>(
    selected_guide: &Guide,
    guides: &'a [Guide],
) -> Option<&'a Guide> {
    let guides = guides
        .iter()
        .filter(|guide| guide.seed_version == selected_guide.seed_version)
        .collect::<Vec<_>>();

    let selected_guide_index =
        guides.iter().position(|guide| *guide == selected_guide)?;

    selected_guide_index
        .checked_add(1)
        .and_then(|index| guides.get(index).copied())
}
