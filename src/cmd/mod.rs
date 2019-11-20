pub mod setup;
use async_std::io;


/// 📢 subcommands 
#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "setup", about = "run to setup sd-card on local pc")]
    Setup(setup::Opt),
}




impl Command {
    pub async fn run(&self) -> io::Result<()> {
        match &self {
            Command::Setup(opt) => opt.run().await
        }
    }
}