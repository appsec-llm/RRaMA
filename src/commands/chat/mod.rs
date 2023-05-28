use clap::Parser;

use crate::commands::common::InferenceArgs;

#[derive(Parser)]
pub struct Chat {
    #[clap(flatten)]
    args: InferenceArgs,

    /// Specify a file from which to read user input, or file to use as a context if combined with QUESTION ('-' for stdin).
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: Option<String>,
}