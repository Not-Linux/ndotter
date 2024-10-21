use std::path::PathBuf;
use relm_derive::Msg;

pub mod traits;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub struct Model {
    pub inversed: bool,
    pub dot_size: u32,
    pub open: bool,
    pub source: Option<PathBuf>,
    pub destination: Option<PathBuf>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            inversed: false,
            dot_size: 1,
            open: false,
            source: None,
            destination: None,
        }
    }
}

structstruck::strike! {
    #[derive(Msg)]
    pub enum Msg {
        Config(pub enum Config {
            Open(bool),
            Inversed(bool),
            SelectFile(pub enum SelectFile {
                Source,
                Destination,
            }),
            ChangeDotSize(u32),
        }),
        Process,
        Quit,
    }
}
