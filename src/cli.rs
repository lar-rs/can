use clap_flags;
use failure::ResultExt;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
use clap_flags::Log;

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

// #[derive(Debug, StructOpt)]
// #[structopt(name = "Socketcan", about = "  🧰 An example of StructOpt usage.")]
// struct Opt {
    // 🔧 Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    // #[structopt(short, long)]
    // debug: bool,

    // interface name
    // we don't want to name it "speed", need to look smart
    // #[structopt(short = "v", long = "velocity", default_value = "42")]
    // speed: f64,
    
//     #[structopt(flatten)]
//     verbosity: clap_flags::Verbosity,
    //  🔧 Output JSON instead of human readable messages
//     #[structopt(name = "json", long = "json")]
//     json: bool,
//       /// The subcommand to run.
//     // pub cmd: Command,
    // ⏱  Message interval in milliseconds
    // #[structopt(name = "interval", long = "interval", default_value = "1000")]
    // interval: u64,
//     /// ⦨  scale signal value
//     #[structopt(name = "scale", long = "scale", default_value = "1.0")]
//     scale: f64,
// }
// 
// impl Cli {
//   /// Initialize a logger.
//   #[inline]
//   pub fn log(&self, name: &str) -> ::Result<()> {
//     self
//       .logger
//       .log(self.verbosity.log_level(), name)
//       .context(::ErrorKind::Log)?;
//     Ok(())
// }

