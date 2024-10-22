use std::{path::PathBuf, sync::Arc};
use notrelm::utils::PersonalizationConfig;
use relm_derive::Msg;

pub struct Model {
    pub config: Arc<PersonalizationConfig>,
    pub inversed: bool,
    pub dot_size: u32,
    pub open: bool,
    pub source: Option<PathBuf>,
    pub destination: Option<PathBuf>,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            config: Arc::new(PersonalizationConfig::load()),
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