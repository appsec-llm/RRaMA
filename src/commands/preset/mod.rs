use clap::Subcommand;

use crate::commands::common::InferenceArgs;

#[derive(Subcommand)]
pub enum Preset {
    /// List all presets
    List,

    /// Add a preset
    Add {
        /// The name of the preset
        #[arg(index = 1, required = true)]
        name: String,

        #[clap(flatten)]
        args: InferenceArgs,
    },

    /// Edit an existing preset
    Edit {
        /// The name of the preset
        #[arg(index = 1, required = true)]
        name: String,

        #[clap(flatten)]
        args: InferenceArgs,
    },

    /// Delete a preset
    Delete {
        /// The name of the preset
        #[arg(index = 1, required = true)]
        name: String,
    },
}

