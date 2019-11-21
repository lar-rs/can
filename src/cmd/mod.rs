pub mod setup;
pub mod server;

use async_std::io;
use structopt::StructOpt;
// use clap_flags;

/// 📢 subcommands 
#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "setup", about = "run to setup sd-card on local pc")]
    Setup(setup::Opt),
    #[structopt(name = "server", about = "can rpc ✇ server run")]
    Server(server::Opt),

}

impl Command {
    pub async fn run(&self) -> io::Result<()> {
        match &self {
            Command::Setup(opt) => opt.run().await,
            Command::Server(opt) => opt.run("can0").await,
        }
    }
}