use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div!["about"]
}
