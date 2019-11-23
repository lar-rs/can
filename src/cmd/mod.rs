// pub mod server;
pub mod setup;
pub mod pipe; 
use async_std::io;
use structopt::StructOpt;
use crate::cli::Args;
// use clap_flags;

/// 📢 subcommands 
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "virtual", about = "activate virtual device")]
    Virt(setup::VCan),
    #[structopt(name = "pipe", about = "run pipe ")]
    Pipe(pipe::Opt),
    // #[structopt(name = "server", about = "can rpc ✇ server run")]
    // Server(server::Opt),

}


impl Cmd {
    pub async fn run(&self,args:&Args) -> io::Result<()> {
        match &self {
            Cmd::Virt(opt) => opt.run(args).await,
            Cmd::Pipe(opt) => opt.run(args).await
            // Command::Server(opt) => opt.run(args).await,
        }
    }
}