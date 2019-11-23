use async_std::io;
// use async_std::path::PathBuf;
use log::info;
use async_std::eprintln;
use crate::error;
use std::process::Command;
// use async_std::process;
// use async_std::fs;
// use crate::rpi as os;

use crate::cli::Args;

// use prettytable::{
    // Table,
    // Row,Cell,
    // table,
    // row,
    // cell,
// };

// use console::{style, Term};
use structopt::StructOpt;
// use crate::can;

pub async fn pican() -> io::Result<()> {
    // http://skpang.co.uk/catalog/pican2-canbus-board-for-raspberry-pi-23-p-1475.html
    //http://skpang.co.uk/catalog/images/raspberrypi/pi_2/PICAN2UG13.pdf
    Ok(())
}


/// Setup pcan driver.
pub async fn run_pcan() -> io::Result<()> {
    Ok(())

    // PCan usb device
    // ```sh
    // modprobe peak_usb 
    // sudo ip link set can0 up type can bitrate 500000
    // cargo test
    // ```
}


/// 🍱  virtual device setup 
#[derive(Debug,StructOpt)]
pub struct VCan{
    /// ⏱  virtual can bitrate settitns
    #[structopt(long = "bitraate", default_value = "500000" )]
    bitrate: u64,
}

impl VCan {
    pub async fn run(&self, cli:&Args) -> io::Result<()> {
        log::info!("activate virtual can device {}",cli.interface());
        Command::new("modprobe") .arg("vcan").spawn().expect("modprobe command failed to start");
        Command::new("ip").arg("link").arg("add").arg("vcan0").arg("type").arg("vcan").spawn().expect("ip link add vcan0 command failed to start");
        Command::new("ip").arg("link").arg("set").arg("vcan0").arg("up").spawn().expect("ip link set vcan0 up command failed to start");
        Ok(())
    }
}
