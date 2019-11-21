use crate::cmd::Command;
// use failure::ResultExt;
use async_std::io;
// use std::io;
// use std::path::PathBuf;
use structopt::StructOpt;
// use clap_flags::Log;


    
    
/// 🚰 greenhous waterpipe controler
#[derive(Debug, StructOpt)]
#[structopt(name = "Socketcan", about = "  🧰 An example of StructOpt usage.")]
pub struct Args {

    #[structopt(short, long)]
    iface: String,
    /// 📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd: Command,
}
/// 🔧 Activate debug mode 
impl Args {
  /// Access the directory name.
  #[inline]
  pub async fn command(&self) -> io::Result<()> {
      self.cmd.run().await
  }
}
