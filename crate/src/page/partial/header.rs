use crate::{
    asset_path,
    generated::css_classes::C,
    image_src, Model, Msg, Page, ScrollHistory, Route,
    Visibility::{self, *},
};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> impl View<Msg> {
    nav![
        id!("header"),
        class![
            C.fixed,
            C.w_full,
            C.z_10,
            C.top_0,
            C.bg_white,
            C.shadow,
            // lg__
            C.lg__shadow_none,
        ],
        div![
            class![
                C.relative,
                C.w_full,
                C.container,
                C.mx_auto,
                C.flex,
                C.flex_wrap,
                C.items_center,
                C.justify_between,
                C.mt_0,
                C.pt_2,
                C.pb_2,
            ],
            div![
                class![
                    C.absolute,
                    C.right_0,
                    C.top_0,
                    C.h_full,
                    C.w_11of12,
                    // lg__
                    C.lg__w_9of12,
                    C.lg__border_b_4,
                    C.lg__border_blue_500,
                ]
            ],
            view_guide_list_toggle().els(),
            view_logo().els(),
            view_menu_toggle().els(),
            view_menu_content(model).els(),
        ]
    ]
}

fn view_logo() -> impl View<Msg> {
    div![
        class![
            C.relative,
            C.flex,
            C.items_center,
            C.pb_px,
            // lg__
            C.lg__pb_0,
            C.lg__mt_1,
            C.lg__pl_16,
        ],
        a![
            class![
                C.w_24,
                C.focus__outline_none,
                // lg__
                C.lg__w_32,
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

fn view_guide_list_toggle() -> impl View<Msg> {
    div![
        class![
            C.relative,
            C.pl_4,
            C.flex,
            // lg__
            C.lg__hidden
        ],
        button![
            id!("view_guide_list_toggle"),
            class![
                C.flex,
                C.items_center,
                C.px_3,
                C.py_2,
                C.font_bold,
                C.border_2,
                C.rounded_full,
                C.tracking_wider,
                C.text_green_500,
                C.hover__text_green_700,
                C.border_green_500,
                C.hover__border_green_700,
                C.appearance_none,
                C.focus__outline_none,
                C.hover__underline,
            ],
            simple_ev(Ev::Click, Msg::ScrollToTop),
            simple_ev(Ev::Click, Msg::ToggleGuideList),
            "Guides",
        ]
    ]
}

fn view_menu_toggle() -> impl View<Msg> {
    div![
        class![
            C.relative,
            C.pr_4,
            C.flex,
            // lg__
            C.lg__hidden
        ],
        button![
            id!("menu_toggle"),
            class![
                C.flex,
                C.items_center,
                C.px_3,
                C.py_2,
                C.font_bold,
                C.border_2,
                C.rounded_full,
                C.tracking_wider,
                C.text_blue_500,
                C.hover__text_blue_700,
                C.border_blue_500,
                C.hover__border_blue_700,
                C.appearance_none,
                C.focus__outline_none,
                C.hover__underline,
            ],
            simple_ev(Ev::Click, Msg::ToggleMenu),
            "Menu",
        ]
    ]
}

fn view_menu_content(model: &Model) -> impl View<Msg> {
    div![
        id!("menu_content"),
        class![
            C.w_full,
            C.relative,
            C.hidden => model.menu_visibility == Hidden,
            C.mt_2,
            C.z_20,
            C.flex,
            C.flex_col,
            C.items_end,
            C.text_right,
            // lg__
            C.lg__flex,
            C.lg__flex_row,
            C.lg__content_center,
            C.lg__items_center,
            C.lg__w_auto,
            C.lg__mt_0,
        ],
        view_links().els(),
        view_github_mark().els(),
    ]
}

fn view_github_mark() -> impl View<Msg> {
    a![
        class![
            C.mt_4,
            C.mb_8,
            C.mr_8,
            // lg__
            C.lg__mx_3,
            C.lg__my_0,
        ],
        attrs!{
            At::Href => "https://github.com/David-OConnor/seed",
        },
        github_mark_svg().els()
    ]
}

fn github_mark_svg() -> impl View<Msg> {
    raw![
        r###############"
            <svg xmlns="http://www.w3.org/2000/svg" id="Layer_1" viewBox="0 0 47.999998 48.000002" width="48" height="48"><style id="style3">.Round_x0020_Corners_x0020_2_x0020_pt{fill:#FFF;stroke:#000;stroke-miterlimit:10}.Live_x0020_Reflect_x0020_X{fill:none}.Bevel_x0020_Soft{fill:url(#SVGID_1_)}.Dusk{fill:#FFF}.Foliage_GS{fill:#FD0}.Pompadour_GS{fill:#44ade2}.Pompadour_GS,.st0{fill-rule:evenodd;clip-rule:evenodd}.st0{fill:#191717}</style><linearGradient id="SVGID_1_" gradientUnits="userSpaceOnUse" x1="-216.625" y1="-385.75" x2="-215.918" y2="-385.043"><stop offset="0" id="stop6" stop-color="#dedfe3"/><stop offset=".174" id="stop8" stop-color="#d8d9dd"/><stop offset=".352" id="stop10" stop-color="#c9cacd"/><stop offset=".532" id="stop12" stop-color="#b4b5b8"/><stop offset=".714" id="stop14" stop-color="#989a9c"/><stop offset=".895" id="stop16" stop-color="#797c7e"/><stop offset="1" id="stop18" stop-color="#656b6c"/></linearGradient><path class="st0" d="M23.928 1.15C11 1.15.514 11.638.514 24.566c0 10.343 6.75 19.105 15.945 22.265 1.148.144 1.58-.574 1.58-1.15v-4.02c-6.465 1.436-7.902-3.16-7.902-3.16-1.005-2.73-2.586-3.45-2.586-3.45-2.154-1.435.144-1.435.144-1.435 2.298.144 3.59 2.442 3.59 2.442 2.156 3.59 5.46 2.586 6.753 2.01.142-1.58.86-2.585 1.435-3.16-5.17-.574-10.63-2.585-10.63-11.635 0-2.585.862-4.596 2.442-6.32-.287-.575-1.005-3.017.288-6.177 0 0 2.01-.574 6.464 2.442 1.866-.574 3.877-.718 5.888-.718 2.01 0 4.022.286 5.89.717 4.453-3.016 6.464-2.442 6.464-2.442 1.293 3.16.43 5.602.287 6.177a9.29 9.29 0 0 1 2.44 6.32c0 9.05-5.458 10.918-10.63 11.492.863.718 1.58 2.155 1.58 4.31v6.464c0 .574.432 1.292 1.58 1.15 9.338-3.16 15.946-11.924 15.946-22.266-.143-12.785-10.63-23.27-23.558-23.27z" id="path20" clip-rule="evenodd" fill="#191717" fill-rule="evenodd"/></svg>
        "###############
    ]
}

fn view_links() -> impl View<Msg> {
    ul![
        class![
            C.justify_end,
            C.items_center,
            // lg__
            C.lg__flex,
        ],
        view_link("Rust Quickstart", "https://github.com/David-OConnor/seed-quickstart").els(),
        view_link("Webpack QS", "https://github.com/MartinKavik/seed-quickstart-webpack").els(),
        view_link("Docs.rs", "https://docs.rs/seed/0.4.1/seed/").els(),
        view_link("Crates.io", "https://crates.io/crates/seed").els(),
        view_link("Awesome List", "https://github.com/MartinKavik/awesome-seed-rs").els(),
//        view_link("GitHub.com", "https://github.com/David-OConnor/seed").els(),
    ]
}

fn view_link(title: &str, link: &str) -> impl View<Msg> {
    li![
        class![
            C.mr_3,
            C.py_2,
            // lg__
            C.lg__py_0,
        ],
        a![
            class![
                C.inline_block,
                C.py_2,
                C.px_4,
                C.text_blue_500,
                C.hover__text_blue_700,
                C.hover__underline,
                C.font_bold,
                C.focus__outline_none,
            ],
            attrs!{
                At::Href => link,
            },
            title
        ]
    ]
}