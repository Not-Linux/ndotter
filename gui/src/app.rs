use std::path::PathBuf;

use gtk::prelude::*;
use gtk::*;
use ndotter_backend::ndot;
use relm::Widget;
use relm_derive::widget;

use crate::utils::traits::*;
use crate::utils::{Config, Model, Msg, SelectFile, Size};
use crate::widgets::notheader::NotHeader;
use crate::widgets::labelled_spin_button::{
    LabelledSpinButton,
    LabelledSpinButtonMsg::*,
};
use crate::widgets::labelled_switch::{
    LabelledSwitch,
    LabelledSwitchMsg::*,
};

#[widget]
impl Widget for App {
    fn model() -> Model {
        Model::default()
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

                gtk::Label {
                    text: "CONVERT IMAGE TO N-DOT",
                    font_family: "Nothing Font (5x7)",
                    font_size: 24,
                    margin_bottom: 10,
                },

                gtk::Box {
                    orientation: Orientation::Horizontal,
                    margin_bottom: 10,

                    gtk::Entry {
                        text: &self.model.source
                            .clone()
                            .map(|p| p.as_path().to_str().unwrap().to_owned())
                            .unwrap_or_default(),
                        placeholder_text: Some("Select source file"),
                        sensitive: false,
                        margin_end: 5,
                        child: {
                            expand: true,
                            fill: true,
                        },
                    },

                    gtk::Button {
                        label: "Select",
    
                        clicked => Msg::Config(
                            Config::SelectFile(
                                SelectFile::Source
                            ),
                        ),
                    },
                },

                gtk::Box {
                    orientation: Orientation::Horizontal,
                    margin_bottom: 10,

                    gtk::Entry {
                        text: &self.model.destination
                            .clone()
                            .map(|p| p.as_path().to_str().unwrap().to_owned())
                            .unwrap_or_default(),
                        placeholder_text: Some("Select destination file"),
                        sensitive: false,
                        margin_end: 5,
                        child: {
                            expand: true,
                            fill: true,
                        },
                    },

                    gtk::Button {
                        label: "Select",
    
                        clicked => Msg::Config(
                            Config::SelectFile(
                                SelectFile::Destination
                            ),
                        ),
                    },
                },

                LabelledSpinButton("Dot size") {
                    tooltip_text: Some("Use black pixels for processing instead of white"),

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