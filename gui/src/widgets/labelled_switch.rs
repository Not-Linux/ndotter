#![allow(unused)]

use gtk::prelude::*;
use gtk::*;
use relm::Widget;
use relm_derive::{widget, Msg};

pub struct LabelledSwitchModel {
    label: &'static str,
    state: bool,
}

#[derive(Msg)]
pub enum LabelledSwitchMsg {
    StateSet(bool),
}

#[widget]
impl Widget for LabelledSwitch {
    fn model(label: &'static str) -> LabelledSwitchModel {
        LabelledSwitchModel {
            label, 
            state: false,
        }
    }

    fn update(&mut self, event: LabelledSwitchMsg) {
        match event {
            LabelledSwitchMsg::StateSet(state) => self.model.state = state,
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

            gtk::Switch {
                active: self.model.state,

                state_set(_, state) => (LabelledSwitchMsg::StateSet(state), Inhibit(false)),
            }
        }
    }
}