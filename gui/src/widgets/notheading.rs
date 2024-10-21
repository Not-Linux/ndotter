#![allow(unused)]

use gtk::prelude::*;
use gtk::*;
use relm::Widget;
use relm_derive::{widget, Msg};
use crate::utils::{traits::*, FontType};

pub struct NotHeadingModel {
    label: &'static str,
    font_type: FontType,
}

#[widget]
impl Widget for NotHeading {
    fn model(data: (&'static str, FontType)) -> NotHeadingModel {
        NotHeadingModel {
            label: data.0,
            font_type: data.1,
        }
    }

    fn update(&mut self, _: ()) {}

    view! {
        gtk::Label {
            text: &match self.model.font_type {
                FontType::Dot => self.model.label.to_uppercase(),
                FontType::Serif => self.model.label.to_owned(),
            },
            font_family: self.model.font_type.to_font_family(),
            font_size: 24,
            margin_bottom: 10,
        },
    }
}