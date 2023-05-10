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

/// The File IO trait
/// This defines the functions used to read and write BV config files
pub trait ConfigFileIO {
    /// This will return a default/empty config should the specified config 
    /// file not exist, which can then be modified and written to disk.
    /// Usage:
    /// config: BvConfig = BvConfig.load_config("FileName");
    fn load_config(config_file: &PathBuf) -> Self;

    /// This will write the config to disk, overwriting any existing config
    /// Usage:
    /// bvconfig.write_config("FileName");
    fn write_config(self: &Self, config_file: &PathBuf) -> Result<()>;
}


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BvConfig {
    pub auto_commit: Option<Bool>,
    pub auto_tag: Option<Bool>,
    pub modules: Option<Vec<ModuleConfig>>,
    pub vcs: Option<SupportedVCS>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleConfig {
    pub auto_commit: Option<Bool>,
    pub auto_tag: Option<Bool>,
    pub current_version: Option<VersionScheme>,
    pub files: Option<Vec<ModuleFile>>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ModuleFile {
    pub path: PathBuf,
    #[serde(default, skip_serializing_if = "is_default_regex_patterns")]
    pub regex: Option<RegexPatterns>,
    pub current_version: VersionScheme,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct RegexPatterns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_pattern: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum VersionScheme {
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

pub enum SupportedVCS {
    Git,
}


impl Default for BvConfig {
    fn default() -> Self {
        Self{
            modules: None
        }
    }
}

impl Default for RegexPatterns {
    fn default() -> Self {
        Self{}
    }
}


impl ConfigFileIO for BvConfig {
    fn load_config(cf: &PathBuf) ->  Self {
        let mut f = match File::open(cf) {
            Ok(f) => f,
            Err(_) => {
                warn!("No config file found, using default configuration");
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

    fn write_config(self: &Self, cf: &PathBuf) -> Result<()> {
        let yaml = serde_yaml::to_string(self).unwrap();
        let mut file = File::create(cf)?;
        Ok(file.write_all(yaml.as_bytes())?)
    }
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


fn is_default_regex_patterns(data: &RegexPatterns) -> bool {
    *data == RegexPatterns::default()
}


