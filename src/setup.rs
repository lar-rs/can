use std::io;
// use async_std::path::PathBuf;
// use lg::info;
// use async_std::eprintln;
// use crate::error;
use std::process::Command;
// use async_std::process;
// use async_std::fs;
// use crate::rpi as os;

// use crate::cli::Args;

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

pub fn pican() -> io::Result<()> {
    // http://skpang.co.uk/catalog/pican2-canbus-board-for-raspberry-pi-23-p-1475.html
    //http://skpang.co.uk/catalog/images/raspberrypi/pi_2/PICAN2UG13.pdf
    Ok(())
}


/// Setup pcan driver.
pub fn run_pcan() -> io::Result<()> {
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
pub struct Opt{
    /// ⏱  virtual can bitrate settitns
    #[structopt(long = "bitraate", default_value = "125000" )]
    bitrate: u64,
    /// Interface name
    #[structopt(name = "interface", default_value = "can0", long = "interface")]
    interface: String,
    /// Activate virtual can device driver long flags (--virt) will be deduced from the field's name
    #[structopt(short, long)]
    virt: bool,
    /// Activate peak usb device driver long flags (--peak) will be deduced from the field's name
    #[structopt(long)]
    peak: bool,
}

/// PCan usb device `Peak`
/// * Linux modul activate:
/// modprobe peak_usb 
/// * Link vcan device: 
/// sudo ip link set can0 up type can bitrate 500000

impl Opt {

    pub fn run(&self) -> io::Result<()> {
        log::info!("activate usp peek can device {}",self.interface);
        if self.virt {
            return self.setup_virtual()
        }
        if self.peak {
            return self.setup_pcan()
        }
        Ok(())
    }
    pub fn setup_pcan(&self) -> io::Result<()> {
        log::info!("Setup USB Peek can device {}",self.interface);

        Command::new("sudo").arg("modprobe") .arg("peak_usb").spawn().expect("modprobe peak_usb command failed");
        // Command::new("sudo").arg("ip").arg("link").arg("add").arg("can0").arg("type").arg("can").spawn().expect("ip link add can0 command failed to start");
        Command::new("sudo").arg("ip").arg("link").arg("set").arg(&self.interface).arg("up").arg("type").arg("can").arg("bitrate").arg(format!("{}",self.bitrate)).spawn().expect("ip link set can0 up command failed to start");
        Ok(())
    }
    pub fn setup_virtual(&self) -> io::Result<()> {
        log::info!("Setup Virtual can device vcan0");
        Command::new("modprobe") .arg("vcan").spawn().expect("modprobe command failed to start");
        Command::new("ip").arg("link").arg("add").arg("vcan0").arg("type").arg("vcan").spawn().expect("ip link add vcan0 command failed to start");
        Command::new("ip").arg("link").arg("set").arg("vcan0").arg("up").spawn().expect("ip link set vcan0 up command failed to start");
        Ok(())
    }
}