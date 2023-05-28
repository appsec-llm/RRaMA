use clap::Subcommand;

#[derive(Subcommand)]
pub enum Template {
    /// List all templates
    List,

    /// Add a template
    Add {
        /// The name of the template
        #[arg(index = 1, required = true)]
        name: String,

        /// The template text
        #[arg(index = 2, required = true)]
        template: String,
    },

    /// Delete a template
    Delete {
        /// The name of the template
        #[arg(index = 1, required = true)]
        name: String,
    },
}