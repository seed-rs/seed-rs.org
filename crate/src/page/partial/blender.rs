use crate::{generated::css_classes::C, Mode, Msg};
use seed::{div, empty, prelude::*, Attrs, C};

pub fn view_for_header(mode: Mode) -> Node<Msg> {
    if mode == Mode::Dark {
        div![C![C.absolute, C.z_40,], common_classes()]
    } else {
        empty![]
    }
}

pub fn view_for_content(mode: Mode) -> Node<Msg> {
    if mode == Mode::Dark {
        div![C![C.fixed, C.z_20,], common_classes()]
    } else {
        empty![]
    }
}

fn common_classes() -> Attrs {
    C![C.inset_0, C.bg_white, C.blend_difference, C.pointer_events_none,]
}
