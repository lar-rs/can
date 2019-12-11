// use failure::ResultExt;
use std::io;
// use std::io;
use std::path::PathBuf;
use structopt::StructOpt;
// use clap_flags::Log;
use crate::pipe;
use crate::setup;

// use crate::cli::Args;
// use clap_flags;

/// 📢 subcommands 
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "setup", about = "setup and activate can driver")]
    Setup(setup::Opt),
     #[structopt(name = "pipe", about = "run pipe ")]
    Pipe(pipe::Opt),
}


impl Cmd {
    pub fn run(&self) -> io::Result<()> {
        match &self {
            Cmd::Setup(opt) => opt.run(),
            Cmd::Pipe(opt) => opt.run(),
            // Cmd::DBus(opt) => opt.run().await,
        }
    }
}
    
/// lscan command argument 
#[derive(Debug, StructOpt)]
#[structopt(name = "lscan", about = "  🧰 linux socket can command line interface usage.")]
pub struct Args {
    ///🔧 watch on dir 
    #[structopt(short = "a", long = "address",  default_value = "tcp:host=192.168.66.59,port=6666")]
    address: String,
    ///🔧 watch on dir 
    #[structopt(short = "d", long = "dir",  default_value = "~/.pwa/mio/ndir")]
    dir: PathBuf,
   /// 📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd: Cmd,
}

/// 🔧 Activate debug mode 
impl Args {
  /// Access the directory name.
  #[inline]
  pub  fn command(&self) -> io::Result<()> {
      self.cmd.run()
  }
}

// #[derive(StructOpt)]
// struct MakeCookie {
//     #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
//     supervising_faerie: String,
//     /// The faerie tree this cookie is being made in.
//     tree: Option<String>,
//     #[structopt(subcommand)]  // Note that we mark a field as a subcommand
//     cmd: Command
// }

// #[derive(StructOpt)]
// enum Command {
//     /// Pound acorns into flour for cookie dough.
//     Pound {
//         acorns: u32
//     },
//     /// Add magical sparkles -- the secret ingredient!
//     Sparkle {
//         #[structopt(short, parse(from_occurrences))]
//         magicality: u64,
//         #[structopt(short)]
//         color: String
//     },
//     Finish(Finish),
// }

// // Subcommand can also be externalized by using a 1-uple enum variant
// #[derive(StructOpt)]
// struct Finish {
//     #[structopt(short)]
//     time: u32,
//     #[structopt(subcommand)]  // Note that we mark a field as a subcommand
//     finish_type: FinishType
// }

// // subsubcommand!
// #[derive(StructOpt)]
// enum FinishType {
//     Glaze {
//         applications: u32
//     },
//     Powder {
//         flavor: String,
//         dips: u32
//     }
// }