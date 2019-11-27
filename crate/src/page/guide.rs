use crate::{
    generated::css_classes::C,
    page::partial::{content_control_panel, guide_list, intro},
    Guide, Model, Msg
};
use seed::{prelude::*, *};

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
        guide_list::view(guide, model).els(),
        view_content(guide, model, show_intro).els(),
    ]
}

fn view_content(
    guide: &Guide,
    model: &Model,
    show_intro: bool,
) -> impl View<Msg> {
    use content_control_panel::Position::{Bottom, Top};
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
        intro::view(show_intro).els(),
        content_control_panel::view(guide, Top, model).els(),
        view_guide_html(guide.html).els(),
        content_control_panel::view(guide, Bottom, model).els(),
        view_netlify_logo().els()
    ]
}

fn view_guide_html(content: &str) -> impl View<Msg> {
    div![
        class![
            // it has to be "markdown-body" so it's content is styled by Github CSS
            C.markdown_body,
        ],
        raw!(content)
    ]
}

fn view_netlify_logo() -> impl View<Msg> {
    a![
        class! {
            C.flex,
            C.mt_5,
            C.justify_center,
        },
        attrs! {
            At::Href => "https://www.netlify.com",
        },
        img![
            class!{
                C.z_auto,
            },
            attrs! {
                At::Src => "https://www.netlify.com/img/global/badges/netlify-light.svg"
            }
        ],
    ]
}
