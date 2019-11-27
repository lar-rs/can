// pub mod server;
pub mod setup;
pub mod pipe; 
use async_std::io;
use structopt::StructOpt;
// use crate::cli::Args;
// use clap_flags;

/// 📢 subcommands 
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "setup", about = "setup and activate can driver")]
    Setup(setup::Opt),
     #[structopt(name = "pipe", about = "run pipe ")]
    Pipe(pipe::Opt),
    // #[structopt(name = "server", about = "can rpc ✇ server run")]
    // Server(server::Opt),

}


impl Cmd {
    pub async fn run(&self) -> io::Result<()> {
        match &self {
            Cmd::Setup(opt) => opt.run().await,
            Cmd::Pipe(opt) => opt.run().await
            // Command::Server(opt) => opt.run(args).await,
        }
    }
}