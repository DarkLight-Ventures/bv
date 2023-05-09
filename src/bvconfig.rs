/*
 *  Copyright (c) 2023 Darklight Ventures
 */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub trait ConfigLoader {
    fn load_config(config_file: &PathBuf) -> Self;
}

pub trait ConfigWriter {
    fn write_config(self: &Self, config_file: &PathBuf);
}


#[derive(Debug, Deserialize, Serialize)]
pub struct BvConfig {
    pub modules: Option<Vec<ModuleConfig>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModuleConfig {
    pub name: Option<String>,
    pub current_version: Option<String>,
}


impl ConfigLoader for BvConfig {
    fn load_config(config_file: &PathBuf) ->  Self {
        Self{
            modules: None,
        }
    }
}

impl ConfigWriter for BvConfig {
    fn write_config(self: &Self, config_file: &PathBuf) {
    }
}