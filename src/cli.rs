use crate::cmd::Cmd;
// use failure::ResultExt;
use async_std::io;
// use std::io;
// use std::path::PathBuf;
use structopt::StructOpt;
// use clap_flags::Log;

    
/// lscan command argument 
#[derive(Debug, StructOpt)]
#[structopt(name = "lscan", about = "  🧰 linux socket can command line interface usage.")]
pub struct Args {

   /// 📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd: Cmd,

   #[structopt()]
    iface: String,
}

/// 🔧 Activate debug mode 
impl Args {
  /// Access the directory name.
  #[inline]
  pub async fn command(&self) -> io::Result<()> {
      self.cmd.run(self).await
  }
  #[inline]
  pub fn interface(&self) -> &str {
     &self.iface
  }
}

#[derive(StructOpt)]
struct MakeCookie {
    #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    cmd: Command
}

#[derive(StructOpt)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound {
        acorns: u32
    },
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        #[structopt(short, parse(from_occurrences))]
        magicality: u64,
        #[structopt(short)]
        color: String
    },
    Finish(Finish),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(StructOpt)]
struct Finish {
    #[structopt(short)]
    time: u32,
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    finish_type: FinishType
}

// subsubcommand!
#[derive(StructOpt)]
enum FinishType {
    Glaze {
        applications: u32
    },
    Powder {
        flavor: String,
        dips: u32
    }
}