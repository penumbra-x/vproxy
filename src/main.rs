pub mod alloc;
#[cfg(target_family = "unix")]
mod daemon;
pub mod error;
mod proxy;
mod update;

use clap::{Args, Parser, Subcommand};
use std::net::SocketAddr;

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
#[command(args_conflicts_with_subcommands = true)]
struct Opt {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run server
    Run(BootArgs),
    /// Start server daemon
    #[cfg(target_family = "unix")]
    Start(BootArgs),
    /// Restart server daemon
    #[cfg(target_family = "unix")]
    Restart(BootArgs),
    /// Stop server daemon
    #[cfg(target_family = "unix")]
    Stop,
    /// Show the server daemon process
    #[cfg(target_family = "unix")]
    PS,
    /// Show the server daemon log
    #[cfg(target_family = "unix")]
    Log,
    /// Update the application
    Update,
}

/// Choose the authentication type
#[derive(Args, Clone)]
pub struct AuthMode {
    /// Authentication username
    #[clap(short, long)]
    pub username: Option<String>,
    /// Authentication password
    #[clap(short, long)]
    pub password: Option<String>,
}

#[derive(Subcommand, Clone)]
pub enum Proxy {
    /// Http server
    Http {
        /// Authentication type
        #[clap(flatten)]
        auth: AuthMode,
    },
    /// Socks5 server
    Socks5 {
        /// Authentication type
        #[clap(flatten)]
        auth: AuthMode,
    },
}

#[derive(Args, Clone)]
pub struct BootArgs {
    /// Debug mode
    #[clap(long, env = "VPROXY_DEBUG")]
    debug: bool,

    /// Bind address
    #[clap(short, long, default_value = "0.0.0.0:1080")]
    bind: SocketAddr,

    /// Connection timeout in seconds
    #[clap(short = 'T', long, default_value = "10")]
    connect_timeout: u64,

    /// Concurrent connections
    #[clap(short, long, default_value = "1024")]
    concurrent: usize,

    /// Ulimit soft limit
    #[cfg(target_family = "unix")]
    #[clap(short, long)]
    ulimit: bool,

    /// IP addresses whitelist, e.g. 47.253.53.46,47.253.81.245
    #[clap(short, long, value_parser, value_delimiter = ',')]
    whitelist: Vec<std::net::IpAddr>,

    /// IP-CIDR, e.g. 2001:db8::/32
    #[clap(short = 'i', long)]
    cidr: Option<cidr::IpCidr>,

    /// IP-CIDR-Range, e.g. 64
    #[clap(short = 'r', long)]
    cidr_range: Option<u8>,

    /// Fallback address
    #[clap(short, long)]
    fallback: Option<std::net::IpAddr>,

    #[clap(subcommand)]
    proxy: Proxy,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();

    match opt.commands {
        Commands::Run(args) => proxy::run(args)?,
        #[cfg(target_family = "unix")]
        Commands::Start(args) => daemon::start(args)?,
        #[cfg(target_family = "unix")]
        Commands::Restart(args) => daemon::restart(args)?,
        #[cfg(target_family = "unix")]
        Commands::Stop => daemon::stop()?,
        #[cfg(target_family = "unix")]
        Commands::PS => daemon::status()?,
        #[cfg(target_family = "unix")]
        Commands::Log => daemon::log()?,
        Commands::Update => update::update()?,
    };

    Ok(())
}
