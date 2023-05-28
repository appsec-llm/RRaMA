use clap::Subcommand;

#[derive(Subcommand)]
pub enum Prompt {
    /// List all named prompts
    List,

    /// Add a named prompt
    Add {
        /// The name of the prompt
        #[arg(index = 1, required = true)]
        name: String,

        /// The prompt text
        #[arg(index = 2, required = true)]
        prompt: String,
    },

    /// Delete a named prompt
    Delete {
        /// The name of the prompt
        #[arg(index = 1, required = true)]
        name: String,
    },
}