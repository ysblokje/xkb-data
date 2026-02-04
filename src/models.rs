// Models as described in the XML files of xkeyboard-config

use crate::{AccessList, ConfigItem};
use serde::Deserialize;

// Keyboard models Section
#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardModels {
    #[serde(rename = "modelList")]
    pub model_list: KeyboardModelList,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeyboardModelList {
    pub model: Vec<KeyboardModel>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "model")]
pub struct KeyboardModel {
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
}

impl AccessList for KeyboardModels {
    type InnerType = KeyboardModel;

    fn get_list(&self) -> &[Self::InnerType] {
        &self.model_list.model
    }

    fn get_mut_list(&mut self) -> &mut [Self::InnerType] {
        &mut self.model_list.model
    }

    fn set_list(&mut self, list: Vec<Self::InnerType>) {
        self.model_list.model = list
    }

    fn new(list: Vec<Self::InnerType>) -> Self {
        Self {
            model_list: KeyboardModelList { model: list },
        }
    }

    fn take_list(self) -> Vec<Self::InnerType> {
        self.model_list.model
    }
}

impl KeyboardModel {
    /// Fetches the name of the keyboard layout.
    pub fn name(&self) -> &str {
        &self.config_item.name
    }

    /// Fetches a description of the layout.
    pub fn description(&self) -> &str {
        &self.config_item.description
    }
}
