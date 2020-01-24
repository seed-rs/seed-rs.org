use crate::{generated::css_classes::C, Mode, Msg};
use seed::{prelude::*, *};

pub fn view_for_header(mode: Mode) -> impl View<Msg> {
    if mode == Mode::Dark {
        div![class![C.absolute, C.z_40,], common_classes()]
    } else {
        empty![]
    }
}

pub fn view_for_content(mode: Mode) -> impl View<Msg> {
    if mode == Mode::Dark {
        div![class![C.fixed, C.z_20,], common_classes()]
    } else {
        empty![]
    }
}

fn common_classes() -> Attrs {
    class![C.inset_0, C.bg_white, C.blend_difference, C.pointer_events_none,]
}
