#![allow(unused)]

use gtk::prelude::*;
use gtk::*;
use relm::Widget;
use relm_derive::{widget, Msg};

pub struct LabelledSpinButtonModel {
    label: &'static str,
    value: u32,
}

#[derive(Msg)]
pub enum LabelledSpinButtonMsg {
    ValueSet(u32),
}

#[widget]
impl Widget for LabelledSpinButton {
    fn model(label: &'static str) -> LabelledSpinButtonModel {
        LabelledSpinButtonModel {
            label, 
            value: 1,
        }
    }

    fn update(&mut self, event: LabelledSpinButtonMsg) {
        match event {
            LabelledSpinButtonMsg::ValueSet(value) => self.model.value = value,
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Horizontal,
            margin_bottom: 10,

            gtk::Label {
                text: self.model.label,
                halign: Align::Start,
                child: {
                    expand: true,
                    fill: true,
                },
            },

            gtk::SpinButton {
                adjustment: &Adjustment::builder()
                    .lower(1.0)
                    .upper(50.0)
                    .step_increment(1.0)
                    .page_increment(5.0)
                    .build(),
                value: self.model.value as f64,

                changed(button) => LabelledSpinButtonMsg::ValueSet(button.value() as u32),
            },
        }
    }
}