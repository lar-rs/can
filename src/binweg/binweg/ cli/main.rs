
use structopt::*;


use async_std::io;
use automata;

/// 🚰 greenhous waterpipe controler
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// 🔧 working directory
    #[structopt(name = "path", long = "path", default_value = ".")]
    path: String,
    /// 📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")] // Note that we mark a field as a subcommand
    pub cmd: Command,
}


fn main() -> io::Result<()> {
    // task::block_on(run())
    automata::hello();
    femme::pretty::Logger::new().start(log::LevelFilter::Trace).unwrap();
    // femme::start(log::LevelFilter::Trace).unwrap();
    log::info!("waterpipe");
    // let config  = waterpipe::config::read(".");
    let args = Cli::from_args();
    task::block_on(command::run(args.cmd))?;
    Ok(())
}


//
