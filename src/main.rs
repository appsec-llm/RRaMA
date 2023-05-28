use clap::{Parser, Subcommand};
use clap::CommandFactory;

use clap_mangen;

mod commands;
use commands::ask::Ask;
use commands::chat::Chat;
use commands::server::Server;
use commands::model::Model;
use commands::prompt::Prompt;
use commands::template::Template;
use commands::preset::Preset;
use commands::common::GlobalArgs;

#[derive(Parser)]
#[command(
    about = "A tool for managing and interacting with LLaMA language models in Rust"
)]
#[command(
    long_about = "RRaMA is a powerful command-line tool that provides a web interface for \
    managing and running inference on LLaMA language models."
)]
#[command(version, arg_required_else_help(true))]
struct Cli {
    #[clap(flatten)]
    global_args: GlobalArgs,

    // Subcommands defined below
    #[command(subcommand)]
    command: Option<Commands>,

    /// Generate man page
    #[arg(long)]
    man: bool,
}


#[derive(Subcommand)]
enum Commands {
    /// Run inference with a single question
    Ask(Ask),

    /// Start a back-and-forth conversation with the model
    Chat(Chat),

    /// Start the rrama web server
    Server(Server),


    // Management and configuration

    /// Model management
    #[command(subcommand)]
    Model(Model),

    /// Prompt management
    #[command(subcommand)]
    Prompt(Prompt),

    /// Template management
    #[command(subcommand)]
    Template(Template),

    /// Preset management
    #[command(subcommand)]
    Preset(Preset),
}


fn main() {
    let _args = Cli::parse();

    if _args.man {
        // Refactor this as we need to generate man pages for each subcommand
        // or maybe find a way to get a single man page for everything
        let cmd = Cli::command();
        let man = clap_mangen::Man::new(cmd);

        let mut buffer: Vec<u8> = Default::default();
        let _ = man.render(&mut buffer);

        println!("{}", String::from_utf8(buffer).unwrap());
        return;
    }

    match _args.command {
        Some(Commands::Server(server)) => {
            server.run();
        },
        // Handle other commands here...
        _ => todo!()
    }
}
