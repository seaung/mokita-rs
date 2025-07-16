use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, author = "seaung")]
pub struct Cli {
    #[arg(short, long)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server {
        #[arg(short, long)]
        config: String,
    },
    Client {
        #[arg(short, long)]
        config: String,
    },
}

pub fn start() {
    let cli = Cli::parse();

    let mut debug: bool;

    match cli.debug {
        0 => debug = true,
        1 => debug = false,
        _ => debug = false,
    }

    match &cli.command {
        Some(Commands::Server { config }) => {
            print!("开始运行服务程序...");
        }
        Some(Commands::Client { config }) => {
            println!("开始运行客户端程序...")
        }
        None => {
            println!("请选择一个程序执行!");
        }
    }
}
