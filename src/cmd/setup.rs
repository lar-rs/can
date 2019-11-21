use async_std::io;
// use async_std::path::PathBuf;
use log::info;
use async_std::eprintln;
use crate::error;
use std::process::Command;
// use async_std::process;
// use async_std::fs;
// use crate::rpi as os;


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

pub async fn run_vcan()  -> io::Result<()> {
    Command::new("modprobe") .arg("vcan").spawn().expect("modprobe command failed to start");
    Command::new("ip").arg("link").arg("add").arg("vcan0").arg("type").arg("vcan").spawn().expect("ip link add vcan0 command failed to start");
    Command::new("ip").arg("link").arg("set").arg("vcan0").arg("up").spawn().expect("ip link set vcan0 up command failed to start");
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


/// setup system can divice may you neet root acces
#[derive(Debug,StructOpt)]
pub struct Opt {
    ///device interface ⥄  [vcan0,can0]
    #[structopt(short = "i", long = "iface")]
    iface: String,
    

}




impl Opt {
    pub async fn run(&self) -> io::Result<()> {
        // let dir = PathBuf::from(&self.path);
        info!("subcommand setup");
        match self.iface.as_str() {
            "vcan" => run_vcan().await,
            _ => {
                eprint!("unknown CAN interface {}",self.iface);
                Ok(())
            }
        }
        // os::setup_config(&dir).await?;
        // os::enable_ssh(&dir).await?;
        // os::enable_wlan(&dir).await?;
        // Ok(())
    }
}

