/*
 *  Copyright (c) 2023 Darklight Ventures
 */

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use tracing::{warn};

pub trait ConfigLoader {
    fn load_config(config_file: &PathBuf) -> Self;
}

pub trait ConfigWriter {
    fn write_config(self: &Self, config_file: &PathBuf) -> Result<()>;
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


impl Default for BvConfig {
    fn default() -> Self {
        Self{
            modules: None
        }
    }
}


impl ConfigLoader for BvConfig {
    fn load_config(cf: &PathBuf) ->  Self {
        let mut f = match File::open(cf) {
            Ok(f) => f,
            Err(_) => {
                warn!("No config file found, using defualt configuration");
                return BvConfig::default()   // Return default Config if file does not exist
            },
        };
    
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(_) => return BvConfig::default(),  // Return default Config if reading fails
        };
    
        match serde_yaml::from_str(&contents) {
            Ok(config) => config,
            Err(_) => BvConfig::default(),  // Return default Config if deserialization fails
        }
    }
}

impl ConfigWriter for BvConfig {
    fn write_config(self: &Self, cf: &PathBuf) -> Result<()> {
        let yaml = serde_yaml::to_string(self).unwrap();
        let mut file = File::create(cf)?;
        Ok(file.write_all(yaml.as_bytes())?)
    }
}