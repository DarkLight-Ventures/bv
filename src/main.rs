/*
 *  Copyright (c) 2023 Darklight Ventures
 */


mod cli;
use crate::cli::*;


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
    //
    match args.command {
        Commands::Add { path } => {
            info!("Adding {path:?}");
        }
    }
}