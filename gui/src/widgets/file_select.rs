#![allow(unused)]

use std::{path::PathBuf, sync::Arc};

use gtk::prelude::*;
use gtk::*;
use relm::Widget;
use relm_derive::{widget, Msg};

pub struct FileSelectModel {
    placeholder: &'static str,
    path: Option<PathBuf>,
}

#[derive(Msg)]
pub enum FileSelectMsg {
    SetPath(Option<PathBuf>),
    InvokeDialog,
}

#[widget]
impl Widget for FileSelect {
    fn model(placeholder: &'static str) -> FileSelectModel {
        FileSelectModel {
            placeholder,
            path: None, 
        }
    }

    fn update(&mut self, event: FileSelectMsg) {
        match event {
            FileSelectMsg::SetPath(path) => self.model.path = path,
            _ => {},
        }
    }

    view! {
        gtk::Box {
            orientation: Orientation::Horizontal,
            margin_bottom: 10,

            gtk::Entry {
                text: &self.model.path
                    .clone()
                    .map(|p| p.as_path().to_str().unwrap().to_owned())
                    .unwrap_or_default(),
                placeholder_text: Some(self.model.placeholder),
                sensitive: false,
                margin_end: 5,
                child: {
                    expand: true,
                    fill: true,
                },
            },

            gtk::Button {
                label: "Select",

                clicked => FileSelectMsg::InvokeDialog,
            },
        },
    }
}