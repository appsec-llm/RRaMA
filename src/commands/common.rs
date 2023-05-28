use std::path::PathBuf;
use clap::Args;

#[derive(Args)]
pub struct GlobalArgs {
        /// Set the verbosity level of the application (0-5)
        #[arg(short, long, value_name = "VERBOSITY_LEVEL", default_value = "0", global = true)]
        verbose: u8,
    
        /// Specify the path to the configuration directory
        #[arg(short, long, value_name = "CONFIG_DIR", default_value = "~/.config/rrama/")]
        config: PathBuf,

        /// Specify the format of the output. Valid options: text, json
        #[arg(long, default_value = "text", global = true)]
        format: String,
}

#[derive(Args)]
pub struct InferenceArgs {
    /// Temperature to use for the preset
    #[arg(short, long, default_value = "0.7")]
    temperature: f32,
    
    /// Number of tokens to generate
    #[arg(short, long, default_value = "2048")]
    max_tokens: u16,
    
    /// Name of the prompt to use
    #[arg(short, long)]
    prompt: Option<String>,
    
    /// Name of the template to use
    #[arg(short = 'T', long)]
    template: Option<String>,
}
