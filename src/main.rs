mod args;
mod controller;
mod database;
mod job;
mod shell;

use args::{Args, Commands};
use clap::Parser;
use controller::Controller;
use rusqlite::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    if args.debug {
        eprintln!("{:?}", &args);
    }
    let mut ctrl = Controller::new(args.debug)?;
    match args.command {
        None if !args.values.is_empty() => ctrl.push(&args.values)?,
        None => ctrl.pop_then_execute(args.num, args.interval)?,
        Some(Commands::Show { filter, tail }) => ctrl.show(filter, tail)?,
        Some(Commands::Revive { id }) => ctrl.revive(id)?,
        Some(Commands::Rm { id }) => ctrl.rm(id)?,
    };
    Ok(())
}
