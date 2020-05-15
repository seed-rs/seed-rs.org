// Clippy complains about `cognitive_complexity` for simple functions with macros.
#![allow(clippy::cognitive_complexity)]

use crate::{
    generated::css_classes::C, page::partial::image, Guide, Msg, SeedVersion,
    Urls, DEFAULT_GUIDE_SLUG, SEED_VERSIONS,
};
use seed::{prelude::*, *};

pub fn view(
    show: bool,
    base_url: &Url,
    guides: &[Guide],
    selected_seed_version: SeedVersion,
) -> Node<Msg> {
    if show {
        div![
            div![
                C![
                    C.mb_8,
                    // sm__
                    C.sm__my_12,
                    C.sm__flex,
                    C.sm__justify_center,
                    C.sm__items_center,
                ],
                view_logo(base_url),
                view_description_and_versions(
                    base_url,
                    guides,
                    selected_seed_version
                ),
            ],
            view_join_forum_chat(),
            view_testimonials(),
        ]
    } else {
        empty![]
    }
}

fn view_logo(base_url: &Url) -> Node<Msg> {
    div![
        C![C.flex,],
        a![
            C![
                C.w_48,
                C.focus__outline_none,
                // lg__
                C.lg__w_64,
            ],
            attrs! {
                At::Href => Urls::new(base_url).home()
            },
            image::seed_logo_svg(),
        ],
    ]
}

fn view_description_and_versions(
    base_url: &Url,
    guides: &[Guide],
    selected_seed_version: SeedVersion,
) -> Node<Msg> {
    div![
        C![C.flex, C.flex_col, C.items_end,],
        view_description(),
        div![
            C![C.flex, C.flex_col, C.mt_2],
            SEED_VERSIONS.iter().map(|version| {
                let version = *version;
                let default_guide = guides
                    .iter()
                    .find(|guide| {
                        guide.slug == DEFAULT_GUIDE_SLUG
                            && guide.seed_version == version.version()
                    })
                    .unwrap();
                a![
                    C![
                        C.flex,
                        C.justify_between,
                        C.items_center,
                        C.rounded_full,
                        C.p_2,
                        C.mt_1,
                        C.cursor_pointer,
                        if version == selected_seed_version {
                            vec![C.text_blue_800, C.bg_green_100]
                        } else {
                            vec![C.text_blue_600]
                        },
                        C.hover__text_blue_800,
                        C.hover__bg_green_100,
                    ],
                    attrs! {
                        At::Href => Urls::new(base_url).guide(default_guide)
                    },
                    span![C![C.font_bold, C.px_2,], version.version()],
                    span![C![C.text_sm, C.px_2,], version.date(),],
                ]
            })
        ]
    ]
}

fn view_description() -> Node<Msg> {
    h2![
        C![
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
    ]
}

fn view_join_forum_chat() -> Node<Msg> {
    div![
        C![
            C.mb_12,
            C.text_center,
            C.text_blue_900,
            C.font_semibold,
            C.text_lg,
            C.whitespace_pre_wrap,
        ],
        span!["You are very welcome to join our "],
        a![
            C![C.text_blue_500, C.hover__text_blue_700, C.hover__underline,],
            "forum",
            attrs! {
                At::Href => "https://seed.discourse.group",
            }
        ],
        span![" and "],
        a![
            C![C.text_blue_500, C.hover__text_blue_700, C.hover__underline,],
            "chat",
            attrs! {
                At::Href => "https://discord.gg/JHHcHp5",
            }
        ],
        span!["!"],
    ]
}

// ------ view testimonials  ------

struct Testimonial {
    quote: &'static str,
    url: &'static str,
    author_image_url: &'static str,
}

fn view_testimonials() -> Node<Msg> {
    let testimonials = vec![
        Testimonial {
            quote: "Awesome, awesome framework!",
            url: "https://github.com/seed-rs/seed/issues/193#issue-479188076",
            author_image_url: "https://avatars.githubusercontent.com/u/16214",
        },
        Testimonial {
            quote: "Seed rocks",
            url: "https://github.com/seed-rs/seed-rs-realworld/issues/1#issuecomment-525413644",
            author_image_url: "https://avatars.githubusercontent.com/u/48671239",
        },
        Testimonial {
            quote: "I’m super stoked about this framework.",
            url: "https://github.com/seed-rs/seed/issues/11#issuecomment-457477672",
            author_image_url: "https://avatars.githubusercontent.com/u/2380740",
        },
        Testimonial {
            quote: "cool, Elm but in Rust!",
            url: "https://github.com/seed-rs/seed/issues/52#issue-412081499",
            author_image_url: "https://avatars.githubusercontent.com/u/38404589",
        },
        Testimonial {
            quote: "this is a pretty cool Rust web framework!",
            url: "https://github.com/seed-rs/seed/issues/16#issue-395014777",
            author_image_url: "https://avatars.githubusercontent.com/u/139499",
        },
        Testimonial {
            quote: "I'm very new to Rust and Seed is the only frontend framework I find accessible.",
            url: "https://github.com/seed-rs/seed/issues/31#issue-403427680",
            author_image_url: "https://avatars.githubusercontent.com/u/45914742",
        },
        Testimonial {
            quote: "this framework looks very promising and the getting-started experience was very smooth thanks to the excellent documentation!",
            url: "https://github.com/seed-rs/seed/issues/5#issue-392002515",
            author_image_url: "https://avatars.githubusercontent.com/u/16122",
        },
        Testimonial {
            quote: "I love it",
            url: "https://github.com/seed-rs/seed/issues/192#issuecomment-518190059",
            author_image_url: "https://avatars.githubusercontent.com/u/16864501",
        },
        Testimonial {
            quote: "It composes really well and feels very Rusty :)",
            url: "https://github.com/seed-rs/seed/issues/193#issuecomment-536255691",
            author_image_url: "https://avatars.githubusercontent.com/u/7584365",
        },
        Testimonial {
            quote: "I like it and hope it become the wide-using wasm webapp dev framework.",
            url: "https://github.com/seed-rs/seed/issues/111#issue-443462538",
            author_image_url: "https://avatars.githubusercontent.com/u/163317",
        }
    ];

    let (testimonials_1, testimonials_2) =
        testimonials.split_at(testimonials.len() / 2);
    div![
        C![
            C.mb_10, // md__
            C.md__mb_0, C.md__flex, // lg__
            C.lg__mb_5,
        ],
        ul![testimonials_1.iter().map(view_testimonial)],
        ul![
            C![
                // md__
                C.md__ml_4,
            ],
            testimonials_2.iter().map(view_testimonial)
        ]
    ]
}

fn view_testimonial(testimonial: &Testimonial) -> Node<Msg> {
    li![a![
        C![
            C.flex,
            C.my_5,
            C.items_center,
            C.hover__underline,
            C.hover__text_green_900,
            C.text_green_700,
        ],
        attrs! {
            At::Href => testimonial.url,
        },
        img![
            C![C.object_contain, C.flex_shrink_0, C.rounded_full,],
            attrs! {
                At::Src => format!("{}{}", testimonial.author_image_url, "?v=4&s=48"),
                At::Height => 48,
                At::Width => 48,
            },
        ],
        div![C![C.mx_2,], testimonial.quote,],
    ],]
}
