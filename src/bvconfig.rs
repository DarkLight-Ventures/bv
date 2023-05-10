/*
 *  Copyright (c) 2023 Darklight Ventures
 */

use anyhow::Result;
use serde::{Serialize, Deserialize, Deserializer};
use serde::ser::{Serializer};
use serde::de::Error;
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
    pub files: Option<Vec<ModuleFile>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleFile {
    pub path: PathBuf,
    pub replace_pattern: Option<String>,
    pub search_pattern: Option<String>,
    pub version: FileVersion,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum FileVersion {
    Semantic(SemanticVersion),
}

#[derive(Debug, PartialEq)]
pub struct SemanticVersion {
    major: i16,
    minor: i16,
    patch: i16,
    pre_release: Option<String>,
    build: Option<String>,
}

impl<'de> Deserialize<'de> for SemanticVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split('+');
        let pre_release_and_rest = parts.next().unwrap();
        let build = parts.next().map(|s| s.to_string());

        if parts.next().is_some() {
            return Err(D::Error::custom("Too many '+' symbols in version string"));
        }

        let mut parts = pre_release_and_rest.split('-');
        let version_and_rest = parts.next().unwrap();
        let pre_release = parts.next().map(|s| s.to_string());

        if parts.next().is_some() {
            return Err(D::Error::custom("Too many '-' symbols in version string"));
        }

        let split: Vec<&str> = version_and_rest.split('.').collect();
        if split.len() != 3 {
            return Err(D::Error::custom("Expected format: [0-9]+.[0-9]+.[0-9]+"));
        }

        let major: i16 = split[0].parse().map_err(D::Error::custom)?;
        let minor: i16 = split[1].parse().map_err(D::Error::custom)?;
        let patch: i16 = split[2].parse().map_err(D::Error::custom)?;

        Ok(Self { major, minor, patch, pre_release, build })
    }
}

impl Serialize for SemanticVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut version_str = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(pre_release) = &self.pre_release {
            version_str = format!("{}-{}", version_str, pre_release);
        }
        if let Some(build) = &self.build {
            version_str = format!("{}+{}", version_str, build);
        }
        serializer.serialize_str(&version_str)
    }
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