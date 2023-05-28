use clap::Parser;

mod server_logic;

/// Start the rrama web server
#[derive(Parser)]
pub struct Server {
    /// Set the port for the web server
    #[arg(short, long, default_value = "8080")]
    //pub port: u16,
    port: u16,

    /// Set the host for the web server
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: std::net::IpAddr,

    /// Run the server in the background
    #[arg(short, long, default_value = "false")]
    background: bool,

    /// Specify the path to the log file
    #[arg(short, long)]
    log: Option<String>,
}

impl Server {
    pub fn run(&self) {
        server_logic::run_server(self.host, self.port);
    }
}