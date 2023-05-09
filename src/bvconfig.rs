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


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BvConfig {
    pub modules: Option<Vec<ModuleConfig>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleConfig {
    pub name: Option<String>,
    pub current_version: Option<String>,
}


impl ConfigLoader for BvConfig {
    fn load_config(cf: &PathBuf) ->  Self {
        if let f = std::fs::File::open(cf) {
            serde_yaml::from_reader(f).expect("Could not read values.")
        } else {
            Self{
                modules: None,
            }
        }
    }
}

impl ConfigWriter for BvConfig {
    fn write_config(self: &Self, cf: &PathBuf) {
    }
}