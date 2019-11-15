// use structopt::StructOpt;
use failure::Fallible;
use std::{
    fs,
    path::PathBuf,
};
use log::info;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
 pub struct  Opt{
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

// use super::dbus;



pub fn run(opt:Opt) -> Fallible<()> {
    info!("run check..");
    // let uart01 = dbus::uart01("tcp:host=192.168.100.133,port=6666");
    // if !opt.path.exists() {
    // super::tui::sparkline::show(opt.path);
        // setup_work_directory(opt.path.clone())?;
    // }
    Ok(())
}
//



impl CanBus for Can {
    fn read_value(&mut self,node : i32,index: i32,sub: u8) -> Result<u32> {
        Ok(0 as u32)
    }
    fn write_u32(&mut self,node:i32,index: i32,sub: u8,value:u32) -> Result<()> {
        Ok(())
    }
    fn write_u16(&mut self,node:i32,index: i32,sub: u8,value:u16) -> Result<()> {
        Ok(())
    }
    fn write_u8(&mut self,node:i32,index: i32,sub: u8,value:u8) -> Result<()> {
        Ok(())
    }
    fn write_bytes(&mut self,node:i32,index: i32,sub: u8,value:Vec<u8>) -> Result<()> {
        Ok(())
    }
    fn read_bytes(&mut self,node:i32,index: i32,sub: u8) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
