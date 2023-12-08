use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "Rust Pulse Mixer")]
#[command(author = "Paulo Guimaraes <paulotechusa@proton.me>")]
#[command(version = option_env!("CARGO_PKG_VERSION"))]
#[command(about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(visible_aliases = ["i"])]
    Install {
        #[arg(short, long)]
        #[arg(help = "Seraches for a package containing the package")]
        search: bool,
    },

    #[command(visible_aliases = ["s"])]
    Search {},

    #[command(visible_aliases = ["r", "uninstall", "delete"])]
    Remove {},

    #[command(visible_aliases = ["U", "update"])]
    Upgrade {},
}

fn main() {
    let _cli = Cli::parse();
}
