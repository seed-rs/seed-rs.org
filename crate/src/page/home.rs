use crate::{
    generated::css_classes::C, image_src, Msg, Page, MAIL_TO_HELLWEB,
    MAIL_TO_KAVIK,
};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    div!["home"]
}