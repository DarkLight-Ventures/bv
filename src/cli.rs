/*
 *  Copyright (c) 2023 Darklight Ventures
 */

// Lints
#![warn(missing_docs)]


use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity as ClapVerbosity};
use std::env;
use std::path::PathBuf;
use tracing::log;
use tracing_subscriber::filter;

/// GetArgs is the trait for command line argument getters
pub trait GetArgs {
    /// get_args: Returns the command line arguments
    fn get_args() -> Self;
}

/// Verbosity is the trait for command line argument verbosity
/// This trait is used to get the verbosity level from the command line arguments
pub trait Verbosity {
    /// get_verbosity: Returns the verbosity level from the command line arguments
    /// let verbosity = self.get_verbosity();
    fn get_verbosity(&self) -> tracing_subscriber::filter::LevelFilter;
}


pub const DEFAULT_TRACE_LEVEL: filter::LevelFilter = filter::LevelFilter::INFO;


/// CliArgs is the main struct for the command line arguments
/// this struct derives from the clap::Parser trait
/// and is used to parse the command line arguments.
/// By default it includes the verbosity flag, which gives `-v` and `-q` flags.
#[derive(Debug, Parser)]
#[command(name = "bv")]
#[command(about = "A version incrementing tool", long_about = None)]
#[command(author = "Kelsey Blackwell & the Darklight Ventures team")]

pub struct CliArgs {
    #[clap(flatten)]
    verbosity: ClapVerbosity<InfoLevel>,

    #[command(subcommand)]
    pub command: Commands,

    #[clap(
        default_value = "bv.yml",
        help = "The config file locationM",
        long,
    )]
    pub config_file: PathBuf,

    #[clap(
        help = "The current version",
        long,
    )]
    pub current_version: Option<String>,

    #[clap(
        help = "The commit message for auto VCS",
        long,
    )]
    pub message: Option<String>,

    #[clap(
        help = "The new version number",
        long,
    )]
    pub new_version: Option<String>,

    #[clap(
        help = "",
        long,
    )]
    pub parse: Option<String>,

    #[clap(
        help = "The version string to replace with",
        long,
    )]
    pub replace: Option<String>,

    #[clap(
        help = "The version string to search for",
        long
    )]
    pub search: Option<String>,

    #[clap(long)]
    pub serialize: Option<String>,

    #[clap(
        help = "The name of the VCS tag",
        long,
    )]
    pub tag_name: Option<String>,

    #[clap(
        help = "The message to tag the commit with",
        long,
    )]
    pub tag_message: Option<String>    
}


#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a component/directory to the config
    Add {
        /// The path(s) of the component
        #[arg(required = true)]
        path: Vec<PathBuf>,
    },
    AutoAdd,
}




// Implementations
impl GetArgs for CliArgs {
    fn get_args() -> Self {
        CliArgs::parse_from(env::args())
    }
}

impl Verbosity for CliArgs {
    fn get_verbosity(&self) -> tracing_subscriber::filter::LevelFilter {
        match self.verbosity.log_level_filter() {
            log::LevelFilter::Off => tracing_subscriber::filter::LevelFilter::OFF,
            log::LevelFilter::Error => tracing_subscriber::filter::LevelFilter::ERROR,
            log::LevelFilter::Warn => tracing_subscriber::filter::LevelFilter::WARN,
            log::LevelFilter::Info => tracing_subscriber::filter::LevelFilter::INFO,
            log::LevelFilter::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            log::LevelFilter::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
        }
    }
}
