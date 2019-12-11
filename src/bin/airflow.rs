
//! Echoes lines read on stdin to stdout.

// use log::info;
// use async_std::io;
// use async_std::prelude::*;
// use async_std::task;
// use async_log::{instrument, span};

// use lscan::can::*;
// use lscan::cli::Args;
// fn main() -> io::Result<()> {

//     task::block_on(async {
//         let stdin = io::stdin();
//         let mut stdout = io::stdout();
//         let mut line = String::new();
//         let mut dev = Can::open("vcan0").unwrap();
//         loop {
//             // Read a line from stdin.
//             let n = stdin.read_line(&mut line).await?;

//             // If this is the end of stdin, return.
//             if n == 0 {
//                 return Ok(());
//             }

//             // Write the line to stdout.
//             stdout.write_all(line.as_bytes()).await?;
//             stdout.flush().await?;
//             line.clear();
//         }
//     })
// }
// use async_log::{instrument, span};
// use log::info;
// use lscan::banner;

// use async_log::span;
// use log::info;
use std::io;
use std::task;
use yansi::Paint;
// use failure::ResultExt;
use std::fs;
use std::path::{PathBuf,Path};
use structopt::StructOpt;
// use clap_flags::Log;
use std::process;

pub enum Sensor  {
    Edinburg,
    Aide,
}




/// ✇ watch signal
#[derive(Debug, StructOpt)]
pub struct Watch{
    ///⏱ interval in seconds
    #[structopt(short = "i", long = "interval",  default_value = "2")]
    interval: usize,
}

impl Watch {
    pub fn run(&self,path:&Path) -> io::Result<()> {
        // let node =
        // Uart::        let mut uart11 = Uart::new(&c,"/com/lar/nodes/Doppelmotor1".to_owned(),dm_interface.to_owned(),1);
        Ok(())
    }
}

/// ✇ airflow sensor settings 
#[derive(Debug, StructOpt)]
pub struct Setting {
   ///⏱ interval in seconds
   #[structopt(short = "i", long = "interval",  default_value = "500")]
   interval: usize, 
    
}

impl Setting {
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if ! path.is_dir() {
            fs::create_dir_all(path)?;
        }
        fs::write(path.join("interval"),format!("{}",self.interval).as_bytes())?;
        Ok(())
    }
}

/// 📢 subcommands
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "average", about = "📢 subcommand to calculate average value")]
    Watch(Watch),
    #[structopt(name = "setup", about = "📢 subcommand to setup sensor uart setting")]
    Setup(Setting)
}


/// ndir sensor command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "ndir", about = "  🧰 ndir sensor interface interface usage.")]
pub struct Args {
    ///🔧 watch on dir
    #[structopt(short = "a", long = "address",  default_value = "tcp:host=192.168.66.59,port=6666")]
    address: String,
     ///🔧 watch on dir
    #[structopt(short = "d", long = "dir",  default_value = "~/.pwa/mio/ndir")]
    dir: PathBuf,
    /// 📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd:Cmd,
}

/// 🔧 Activate debug mode
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn command(&self) -> io::Result<()> {

        let pid = self.dir.join("pid");
        if pid.is_file() {
            log::warn!("sensor is busy pid {}",fs::read_to_string(&pid)?);
            return Ok(())
        }
        let path = self.dir.clone();
        match &self.cmd {
            Cmd::Watch(watch) => {
                fs::write(&pid,&format!("{}",process::id()));
                watch.run(&self.dir);
                fs::remove_file(&pid);
                // can::io::average_signal().await?;
            },
            Cmd::Setup(setup) => {
                setup.save(&path);
                //
                // let uart =  fs::read_to_string(path.join("uart")).await?.parse::<u8>()?;
                // let signal_count = fs::read_to_string(path.join("counter")).await?.parse::<usize>()?;
                // let bitrate = fs::read_to_string(path.join("bitrate")).await?.parse::<u32>().unwrap_or(9600);
                // let format = fs::read_to_string(path.join("format")).await?;
                // let interval = fs::read_to_string(path.join("interval")).await?.parse::<u32>().unwrap_or(500);
            }
        }
        Ok(())
    }
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    println!("{}",Paint::blue(can::banner::NAME));
    args.command()

}


