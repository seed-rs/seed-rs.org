use crate::{generated::css_classes::C, Msg};
use seed::{attrs, path, prelude::*, raw, svg, C};

pub fn seed_logo_svg() -> Vec<Node<Msg>> {
    raw![include_str!("../../../../seed_branding/seed_logo.min.svg")]
}

pub fn spinner_svg() -> Vec<Node<Msg>> {
    raw![
        r#"
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid"><path stroke="none" d="M10 50A40 40 0 0 0 90 50A40 42 0 0 1 10 50" fill="currentColor"></path></svg>
        "#
    ]
}

pub fn github_mark_svg() -> Vec<Node<Msg>> {
    raw![
        r###############"
            <svg xmlns="http://www.w3.org/2000/svg" id="Layer_1" viewBox="0 0 47.999998 48.000002" width="48" height="48"><style id="style3">.Round_x0020_Corners_x0020_2_x0020_pt{fill:#FFF;stroke:#000;stroke-miterlimit:10}.Live_x0020_Reflect_x0020_X{fill:none}.Bevel_x0020_Soft{fill:url(#SVGID_1_)}.Dusk{fill:#FFF}.Foliage_GS{fill:#FD0}.Pompadour_GS{fill:#44ade2}.Pompadour_GS,.st0{fill-rule:evenodd;clip-rule:evenodd}.st0{fill:#191717}</style><linearGradient id="SVGID_1_" gradientUnits="userSpaceOnUse" x1="-216.625" y1="-385.75" x2="-215.918" y2="-385.043"><stop offset="0" id="stop6" stop-color="#dedfe3"/><stop offset=".174" id="stop8" stop-color="#d8d9dd"/><stop offset=".352" id="stop10" stop-color="#c9cacd"/><stop offset=".532" id="stop12" stop-color="#b4b5b8"/><stop offset=".714" id="stop14" stop-color="#989a9c"/><stop offset=".895" id="stop16" stop-color="#797c7e"/><stop offset="1" id="stop18" stop-color="#656b6c"/></linearGradient><path class="st0" d="M23.928 1.15C11 1.15.514 11.638.514 24.566c0 10.343 6.75 19.105 15.945 22.265 1.148.144 1.58-.574 1.58-1.15v-4.02c-6.465 1.436-7.902-3.16-7.902-3.16-1.005-2.73-2.586-3.45-2.586-3.45-2.154-1.435.144-1.435.144-1.435 2.298.144 3.59 2.442 3.59 2.442 2.156 3.59 5.46 2.586 6.753 2.01.142-1.58.86-2.585 1.435-3.16-5.17-.574-10.63-2.585-10.63-11.635 0-2.585.862-4.596 2.442-6.32-.287-.575-1.005-3.017.288-6.177 0 0 2.01-.574 6.464 2.442 1.866-.574 3.877-.718 5.888-.718 2.01 0 4.022.286 5.89.717 4.453-3.016 6.464-2.442 6.464-2.442 1.293 3.16.43 5.602.287 6.177a9.29 9.29 0 0 1 2.44 6.32c0 9.05-5.458 10.918-10.63 11.492.863.718 1.58 2.155 1.58 4.31v6.464c0 .574.432 1.292 1.58 1.15 9.338-3.16 15.946-11.924 15.946-22.266-.143-12.785-10.63-23.27-23.558-23.27z" id="path20" clip-rule="evenodd" fill="#191717" fill-rule="evenodd"/></svg>
        "###############
    ]
}

pub fn next_icon_svg() -> Vec<Node<Msg>> {
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

pub fn search_icon_svg() -> Node<Msg> {
    svg![
        C![
            C.fill_current,
            C.pointer_events_none,
            C.text_green_800,
            C.w_4,
            C.h_4,
        ],
        attrs! {
            At::ViewBox => "0 0 20 20",
        },
        path![attrs! {
            At::D => "M12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12zM12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12z",
        }]
    ]
}

pub fn info_icon_svg() -> Node<Msg> {
    svg![
        C![C.fill_current, C.w_4, C.h_4, C.mr_2,],
        attrs! {
            At::ViewBox => "0 0 20 20",
        },
        path![attrs! {
            At::D => "M12.432 0c1.34 0 2.01.912 2.01 1.957 0 1.305-1.164 2.512-2.679 2.512-1.269 0-2.009-.75-1.974-1.99C9.789 1.436 10.67 0 12.432 0zM8.309 20c-1.058 0-1.833-.652-1.093-3.524l1.214-5.092c.211-.814.246-1.141 0-1.141-.317 0-1.689.562-2.502 1.117l-.528-.88c2.572-2.186 5.531-3.467 6.801-3.467 1.057 0 1.233 1.273.705 3.23l-1.391 5.352c-.246.945-.141 1.271.106 1.271.317 0 1.357-.392 2.379-1.207l.6.814C12.098 19.02 9.365 20 8.309 20z",
        }]
    ]
}
