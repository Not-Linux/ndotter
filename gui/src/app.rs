use std::path::PathBuf;
use std::fs::read_to_string;
use gtk::prelude::*;
use gtk::*;
use ndotter_backend::ndot;
use relm::Widget;
use relm_derive::widget;
use toml::Value;

use crate::utils::{traits::*, FontType};
use crate::utils::{Config, Model, Msg, SelectFile, Size};
use crate::widgets::notheading::NotHeading;
use crate::widgets::{
    notheader::NotHeader,
    file_select::{FileSelect, FileSelectMsg::*},
    labelled_spin_button::{LabelledSpinButton, LabelledSpinButtonMsg::*},
    labelled_switch::{LabelledSwitch, LabelledSwitchMsg::*},
};

#[widget]
impl Widget for App {
    fn model() -> Model {
        let mut model = Model::default();
        
        if let Ok(toml) = read_to_string(format!("{}/.config/not-linux/personalization.toml", env!("HOME"))) {
            if let Ok(main_table) = toml.parse::<toml::Table>() {
                if let Some(personalization) = main_table.get("personalization") {
                    model.font_type = match personalization.get("font_type") {
                        Some(Value::String(s)) if s.as_str() == "Serif" => FontType::Serif,
                        _ => FontType::Dot,
                    };
                }
            }
        }

        dbg!(&model.font_type);

        model
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Process => {
                if let Some(ref source) = self.model.source {
                    if let Err(e) = ndot(
                        source, 
                        self.model.destination.clone(),
                        self.model.inversed, 
                        self.model.open, 
                        self.model.dot_size,
                    ) {
                        self.message_dialog(e.to_string().as_str(), MessageType::Error, false);
                    }

                    self.message_dialog("Finished", MessageType::Info, false);
                } else {
                    self.message_dialog("Source file not chosen!", MessageType::Error, false);
                }
            },
            Msg::Config(config) => match config {
                Config::Inversed(i) => self.model.inversed = i,
                Config::Open(i) => self.model.open = i,
                Config::ChangeDotSize(size) => self.model.dot_size = size,
                Config::SelectFile(select) => {
                    let filter = FileFilter::new();

                    match select {
                        SelectFile::Source => {
                            filter.set_name(Some("Image files"));
                            filter.add_mime_type("image/png");
                            filter.add_mime_type("image/jpeg");
                            filter.add_mime_type("image/jpg");
                            filter.add_mime_type("image/gif");

                            let path = self.open_dialog(FileChooserAction::Open, &[filter]);

                            if path.is_some() {
                                self.streams.src_chooser.emit(SetPath(path.clone()));
                                self.model.source = path;
                            }
                        },
                        SelectFile::Destination => {
                            filter.set_name(Some("SVG file"));
                            filter.add_mime_type("image/svg");

                            let path = self.open_dialog(FileChooserAction::Save, &[filter])
                                .map(|mut p| { 
                                    p.set_extension("svg"); 
                                    p 
                                });

                            if path.is_some() {
                                self.streams.dst_chooser.emit(SetPath(path.clone()));
                                self.model.destination = path;
                            }
                        },
                    }
                }
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name = "window"]
        gtk::Window {
            title: "ndotter",
            position: WindowPosition::Center,
            resizable: false,
            size: Size { width: 360, height: 270 },
            titlebar: view! { 
                NotHeader("ndotter") {},
            },

            gtk::Box {
                orientation: Orientation::Vertical,
                halign: Align::Center,
                margin: 20,

                NotHeading("Convert image to N-Dot", self.model.font_type) {},

                #[name = "src_chooser"]
                FileSelect("Select source file") {
                    InvokeDialog => Msg::Config(
                        Config::SelectFile(
                            SelectFile::Source
                        ),
                    ),
                },

                #[name = "dst_chooser"]
                FileSelect("Select destination file") {
                    InvokeDialog => Msg::Config(
                        Config::SelectFile(
                            SelectFile::Destination
                        ),
                    ),
                },

                LabelledSpinButton("Dot size") {
                    tooltip_text: Some("Size of each N-dot (minimal is 1)"),

                    ValueSet(value) => Msg::Config(
                        Config::ChangeDotSize(value)
                    )
                },

                LabelledSwitch("Inversed") {
                    tooltip_text: Some("Use black pixels for processing instead of white"),

                    StateSet(state) => Msg::Config(
                        Config::Inversed(state)
                    ),
                },

                LabelledSwitch("Open") {
                    tooltip_text: Some("Open processed image after finishing"),

                    StateSet(state) => Msg::Config(
                        Config::Open(state)
                    ),
                },

                gtk::Button {
                    label: "Process",
                    margin_bottom: 15,

                    clicked => Msg::Process,
                },
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

impl App {
    pub fn open_dialog(
        &self,
        action: FileChooserAction,
        filters: &[FileFilter],
    ) -> Option<PathBuf> {
        let dialog = FileChooserDialog::new(
            Some("Open a file"), 
            Some(&self.widgets.window), 
            action,
        );

        for filter in filters {
            dialog.add_filter(filter);
        }

        dialog.add_button("Cancel", ResponseType::Cancel);
        dialog.add_button("Accept", ResponseType::Accept);

        let result = dialog.run();
        let uri = dialog.uri();
        if result != ResponseType::Accept {
            dialog.close();
            return None;
        }

        dialog.close();

        if let Some(uri) = uri {
            let mut uri = uri.to_string();
            uri = uri
                .trim_start_matches("file:/")
                .trim_start_matches('/')
                .to_owned();

            if cfg!(target_family = "unix") {
                uri = format!("/{}", uri);
            }

            match urlencoding::decode(&uri) {
                Ok(decoded) => {
                    uri = decoded.into_owned();
                },
                Err(e) => self.message_dialog(e.to_string().as_str(), MessageType::Error, true),
            }
        
            return Some(PathBuf::from(uri));
        }

        None
    }

    pub fn message_dialog(
        &self,
        message: &str,
        message_type: MessageType, 
        fatal: bool,
    ) {
        let dialog = MessageDialog::new(
            Some(&self.widgets.window),
            DialogFlags::all(),
            message_type,
            ButtonsType::Ok,
            message,
        );

        dialog.show();
        
        if fatal {
            dialog.connect_response(|_, _| gtk::main_quit());
        } else {
            dialog.connect_response(|dialog, _| dialog.emit_close());
        }
    }
}