use clap::Parser;

use crate::commands::common::InferenceArgs;

#[derive(Parser)]
pub struct Ask {
    /// The question to ask
    #[arg(index = 1, required = false)]
    question: Option<String>,

    #[clap(flatten)]
    args: InferenceArgs,

    /// Specify a file from which to read user input, or file to use as a context if combined with QUESTION ('-' for stdin).
    #[arg(short, long, value_name = "INPUT_FILE")]
    input: Option<String>,
        
    /// Specify a file to which to write the model's output, or '-' to write to stdout.
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<String>,
}