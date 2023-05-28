use clap::Subcommand;

/// Handle model-related operations
#[derive(Subcommand)]
pub enum Model {
    /// List all local models
    List,

    /// Search for a model in HuggingFace
    Search {
        /// The query to search for
        #[arg(index = 1, required = true)]
        query: String,

        /// Show invalid
        #[arg(short, long, default_value = "false")]
        invalid: bool,

        /// The maximum number of results to return
        #[arg(short, long, default_value = "10")]
        limit: u16,

        /// The page of results to return
        #[arg(short, long, default_value = "1")]
        page: u16,

        /// The field by which to sort results
        #[arg(short, long, default_value = "lastModified")]
        sort: String,

        /// The order in which to sort results
        #[arg(short, long, default_value = "desc")]
        order: String,

        /// The type of model to search for (valid: 'all', 'torch', 'ggml')
        #[arg(short, long, default_value = "all")]
        type_: String,
    },

    /// Download a model from HuggingFace
    Download {
        /// The model ID to download
        #[arg(index = 1, required = true)]
        id: String,

        /// Force download even if the model already exists
        #[arg(short, long, default_value = "false")]
        force: bool,

        /// Local name to use for the model
        #[arg(short, long)]
        name: Option<String>,

        /// Quantization level to use for the model
        #[arg(short, long, default_value = "5_1")]
        quantization: String,
    },

    /// Delete a local model
    Delete {
        /// The model ID to delete
        #[arg(index = 1, required = true)]
        id: String,
    },

    /// Show information about a local model
    Info {
        /// The model ID to show information about
        #[arg(index = 1, required = true)]
        id: String,
    },

    /// Configure a local model
    Configure {
        /// The model ID to configure
        #[arg(index = 1, required = true)]
        id: String,

        /// Temperature to use for inference
        #[arg(short, long, default_value = "1.0")]
        temperature: f32,

        /// Number of tokens to generate
        #[arg(short, long, default_value = "2048")]
        max_tokens: u16,

        /// Name of the prompt to use
        #[arg(short, long)]
        prompt: Option<String>,

        /// Name of the template to use
        #[arg(short = 'T', long, default_value = "default")]
        template: String,
    },
}