use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, author = "seaung")]
pub struct Cli {
    #[arg(short, long)]
    config: std::path::PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server,
    Client,
}

pub fn start_server() {
    let cli = Cli::parse();

    let config_path = &cli.config;

    if !config_path.exists() {
        panic!("config file not found...");
    }

    match cli.command {
        Some(Commands::Server) => {
            println!("start server programming deamon...");
        }
        Some(Commands::Client) => {
            println!("start client programming daemon...");
        }
        None => {
            eprintln!("No Command provide. run mokita-rs --help please...")
        }
    }
}
