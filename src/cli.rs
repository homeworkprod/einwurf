/*
 * Copyright 2022-2023 Jochen Kupperschmidt
 * License: MIT
 */

use clap::Parser;
use std::path::PathBuf;

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(about, author, version)]
pub(crate) struct Args {
    /// Configuration filename
    #[clap(short = 'c', long = "config")]
    pub config_filename: PathBuf,
}

pub(crate) fn parse_args() -> Args {
    Args::parse()
}
