use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub debug: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(short, long, default_value_t = 1)]
    pub num: usize,
    #[arg(short, long, default_value_t = 1)]
    pub interval: u64,
    pub values: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Show {
        #[arg(short, long, default_value_t = String::new())]
        filter: String,
        #[arg(short, long, default_value_t = 0)]
        tail: usize,
    },
    Revive {
        id: usize,
    },
    Rm {
        id: usize,
    },
}
