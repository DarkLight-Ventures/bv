/*
 *  Copyright (c) 2023 Darklight Ventures
 */

mod bvconfig;
use crate::bvconfig::*;

mod cli;
use crate::cli::*;

mod modules;
use crate::modules::*;


use std::path::PathBuf;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{fmt, reload, prelude::*};


fn main() {
    // Before we do anything else, set up tracing at default levels.
    let (filter, reload_handle) = reload::Layer::new(DEFAULT_TRACE_LEVEL);
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();
    trace!("Logging & Tracing initialized");
    // Get the cli args
    let args = CliArgs::get_args();
    // Set the tracing verbosity level from the cli args
    reload_handle.modify(|filter| *filter = args.get_verbosity()).ok();
    trace!("The verbosity level received in cli_args is: {:?}", args.get_verbosity());
    // Get a configuration object
    let current_config = BvConfig::load_config(&args.config_file);
    //
    let matched = match args.command {
        Commands::Add { path } => add_module(&args),
        Commands::AutoAdd =>  auto_add_modules(&args),
    }
    // Write the config out
    current_config.write_config(&args.config_file);
}



