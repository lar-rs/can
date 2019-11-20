use async_std::io;
use async_std::path::PathBuf;
use log::info;
// use async_std::fs;
use crate::rpi as os;


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

///run to setup sd-card on local pc
#[derive(Debug,StructOpt)]
pub struct Opt {
    ///sd boot directory mount path
    #[structopt(short = "p", long = "path")]
    path: String,
}



impl Opt {
    pub async fn run(&self) -> io::Result<()> {
        let dir = PathBuf::from(&self.path);
        info!("run subcommand setup");
        os::setup_config(&dir).await?;
        os::enable_ssh(&dir).await?;
        os::enable_wlan(&dir).await?;
        Ok(())
    }
}