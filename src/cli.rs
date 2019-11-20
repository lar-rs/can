use clap_flags;
// use failure::ResultExt;
// use std::io;
// use std::path::PathBuf;
use structopt::StructOpt;
// use clap_flags::Log;

// use structopt::StructOpt;
// use futures::prelude::*;

///  ✇ serve tcp/udp controller on net
#[derive(Debug, StructOpt)]
pub struct Address {
    /// 📪  linux socket file or ip addresse
    #[structopt(short = "p", long = "port", env = "PORT", default_value = "8080")]
    port: u16,
     /// 📪  linux socket file or ip addresse
    #[structopt(short = "a", long = "address", default_value = "127.0.0.1")]
    address: String,
}

/// The various kinds of commands that `waretpipe` can execute.
#[derive(Debug, StructOpt)]
pub enum Command {
    Pipe{
        /// 🔧 Activate json mode
        // short and long flags (-d, --debug) will be deduced from the field's name
        #[structopt(short, long)]
        json: bool,
    },
     ///  ✇ serve controller on net
    #[structopt(name = "ipc", about = " ✇ start net controller")]
    Ipc{
        /// 📪  linux socket file patn
        #[structopt(name = "path", long = "📪  linux socket file path")]
        path: String,
    },
    Tcp(Address),
    Udp(Address),
}
/// Port to listen on.
    
    
/// 🚰 greenhous waterpipe controler
#[derive(Debug, StructOpt)]
#[structopt(name = "Socketcan", about = "  🧰 An example of StructOpt usage.")]
pub struct Args {

    /// ⥄‍  device [vcan0,can0]
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
  pub fn iface(&self) -> &str {
      &self.iface
  }
}
