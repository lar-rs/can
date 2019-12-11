//! Echoes lines read on stdin to stdout.
use std::io;
use yansi::Paint;
// use failure::ResultExt;
// use std::io;
// use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;
// use std::time::Duration;
use std::path::{PathBuf};
// use embedded_hal::digital::v2::{InputPin,OutputPin};

// use clap_flags::Log;
use can;


// use crate::cli::Args;
// use clap_flags;

// /// 📢 subcommands 
// #[derive(Debug, StructOpt)]
// pub enum Cmd {
//     #[structopt(name = "setup", about = "setup and activate can driver")]
//     Setup(setup::Opt),
//      #[structopt(name = "pipe", about = "run pipe ")]
//     Pipe(pipe::Opt),
// }


// impl Cmd {
//     pub async fn run(&self) -> io::Result<()> {
//         match &self {
//             Cmd::Setup(opt) => opt.run().await,
//             Cmd::Pipe(opt) => opt.run().await,
//             // Cmd::DBus(opt) => opt.run().await,
//         }
//     }
// }

/// ndir sensor command argument 
#[derive(Debug,StructOpt)]
#[structopt(name = "ndir", about = "  🧰 ndir sensor interface interface usage.")]
pub struct Args {
    ///🔧 watch on dir 
    #[structopt(short = "d", long = "dir",  default_value = "~/.pwa/mio/dbus")]
    dir: PathBuf,
    /// dbus address
    #[structopt(name = "address", default_value = "tcp:host=192.168.66.59,port=6666", long = "address")]
    address: String,
}

/// 🔧 Activate debug mode 
impl Args {
    
  /// Access the directory name.
    #[inline]
    pub fn start(&self) -> io::Result<()> {
        log::info!("{:?} - {} digital mio driver",&self.dir,&self.address);
        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        // let path = PathBuf::from("")
        if !self.dir.is_dir(){
            fs::create_dir_all(&self.dir)?;
        }
        can::io::start(&self.dir,self.address.as_str())?;
        Ok(())
   
    }
}




#[paw::main]
fn main(args: Args) -> io::Result<()> {
    println!("{}",Paint::blue(can::banner::NAME));
    // setup_logger();
    args.start()
}


