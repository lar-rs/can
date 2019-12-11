//! Aide sensor

#[macro_use]
extern crate lazy_static;
use std::fs;
use std::fs::File;
use std::io::LineWriter;
use std::io;
use std::io::prelude::*;
use std::path::{PathBuf,Path};
use structopt::StructOpt;
// use clap_flags::Log;
use std::process;
// use std::stream;
// use std::prelude::*;
use std::time;
// use can::error::CanError;
// use regex::{Regex,RegexSetBuilder};
use regex::Regex;
// use lazy_static::lazy_static;
// use std::time::Duration;
// use std::str::FromStr;
use std::time::SystemTime;
// use async_std::println;

// use std::collections::HashSet;
pub enum Sensor  {
    Edinburg,
    Aide,
}

fn extract_fsr(input: &str) -> Option<f32> {
    lazy_static! {
                                            // N 0.0384                0.0000        0.0000      0.00        0.0000       25764             997.2           0
        static ref RE: Regex = Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1,5}.\d{1}) \d{1}").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("fsr").map(|fsr| fsr.as_str().parse::<f32>().unwrap_or(0.0))
    })
}


// Testdate edinburgh
// const TEST_DATA: &'static str  =  "N 0.0414 0.0000 0.0000 0.00 0.0000 22942 992.6";
// Regex::new(r"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{1}.\d{4}) \d{1}").unwrap()*
// -- Listenmakros für Aufzählungen
// #define BAUD_RATES(g,f,d) \
// 	g(9600,   d) \
// 	f(1200,   d) \
// 	f(2400,   d) \
// 	f(4800,   d) \
// 	f(9600,   d) \
// 	f(19200,  d) \
// 	f(38400,  d) \
// 	f(57600,  d) \
// 	f(115200, no)
pub struct Signal{
    fsr: f64,
    ppm: f64,
    dig: u64,
}
impl std::fmt::Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fsr:{} dig:{} ppm:{}", self.fsr, self.ppm, self.dig)
    }
}


pub fn average(path:&Path, address:&str) -> io::Result<()> {
    println!("DIR:{:?}",path);
    let c = can::io::connection(address)?;
    let node = fs::read_to_string(path.join("node"))?;
    let uart =  fs::read_to_string(path.join("uart"))?.parse::<u8>().unwrap_or(1);
    let mut uart = can::io::Uart::new(&c,node,"com.lar.nodes.Doppelmotor3".to_owned(),uart);
    let counter = fs::read_to_string(path.join("counter"))?.parse::<usize>().unwrap_or(20);
    let bitrate = fs::read_to_string(path.join("bitrate"))?.parse::<u32>().unwrap_or(9600);
    // let format = fs::read_to_string(path.join("format")).await?;
    let interval = time::Duration::from_millis(fs::read_to_string(path.join("interval"))?.parse::<u64>().unwrap_or(500));
    let file = File::create(path.join("average"))?;
    let mut average = LineWriter::new(file);
    let mut value = File::create(path.join("value"))?;
    // let mut average = fs::File::create(path.join("average")).await?;
    // let mut value = fs::File::create(path.join("value")).await?;
    let now = SystemTime::now();
    uart.set_bitrate(bitrate)?;
    let mut buffer = String::from("");
    for _c in 0..counter {
        buffer.push_str(uart.read_string()?.as_str());
        println!("uart string {}",buffer);
        let values:Vec<_> = buffer.split(r#"\r\n"#).collect();
        for v in values {
            println!("data: {}",v);
            if let Some(fsr) = extract_fsr(v) {
                let tm = now.elapsed().unwrap();
                average.write_all(format!("{},{}",tm.as_millis(),fsr).as_bytes())?;
                // write!(signal,"{},{}\n",tm.as_millis(),fsr)?;
                value.write_all(format!("{}",fsr).as_bytes())?;
                println!("DATA:{},{}",tm.as_millis(),fsr);

            }else {
                // let re = Regex::new(r#"N (?P<fsr>\d{1}.\d{4}) \d{1}.\d{4} \d{1}.\d{4} \d{1}.\d{2} \d{1}.\d{4} (?P<dig>\d{5}) (?P<ppm>\d{3}.\d{1}) \d{1}"#).unwrap();
                // for caps in re.captures(v) {
                    // println!("FSR: {:?}, DIG: {:?}",
                        //  &caps["fsr"], &caps["dig"]).await;
                // }
                log::warn!("{} wrong format",&v);
            }
        }
        std::thread::sleep(interval);
        buffer.truncate(0);
    }
    average.flush()?;
    Ok(())
}


/// ✇ average signal
#[derive(Debug, StructOpt)]
pub struct Average{
    /// ⏱  interval in seconds
    #[structopt(name = "count", long = "count", default_value = "20")]
    count: usize,
}

impl Average {
    pub fn run(&self,dir:&Path,address:&str) -> io::Result<()> {
        average(dir,address)?;
        Ok(())
    }
}

/// ✇ average signal
#[derive(Debug, StructOpt)]
pub struct Setting {
   ///📬 node address
    #[structopt(short = "n", long = "node",  default_value = "/com/lar/nodes/Doppelmotor1")]
    node: String,
    ///📍 uart select [0..1]
    #[structopt(short = "u", long = "uart",  default_value = "1")]
    uart: u8,
    //⏱ interval in seconds
    #[structopt(short = "i", long = "interval",  default_value = "500")]
    interval: usize,
    //🔌 uart port bitrate setting
    #[structopt(short = "b", long = "bitrate",  default_value = "57600")]
    bitrate: usize,
    ///🔧 average counter
    #[structopt(short = "c", long = "counter",  default_value = "20")]
    counter: usize,
}
impl Setting {
    pub fn save(&self, path: &Path) -> io::Result<()> {
        if ! path.is_dir(){
            fs::create_dir_all(path)?;
        }
        fs::write(path.join("node"),self.node.as_bytes())?;
        fs::write(path.join("uart"),format!("{}",self.uart).as_bytes())?;
        fs::write(path.join("bitrate"),format!("{}",self.bitrate).as_bytes())?;
        fs::write(path.join("counter"),format!("{}",self.counter).as_bytes())?;
        fs::write(path.join("interval"),format!("{}",self.interval).as_bytes())?;
        Ok(())
    }
}

/// 📢 subcommands
#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(name = "average", about = "📢 subcommand to calculate average value")]
    Average(Average),
    #[structopt(name = "setup", about = "📢 subcommand to setup sensor uart setting")]
    Setup(Setting),
    #[structopt(name = "clean", about = "📢 subcommand to clean pid")]
    Clean,


}


///AIDE sensor command argument
#[derive(Debug, StructOpt)]
#[structopt(name = "ndir", about = "  🧰 ndir sensor interface interface usage.")]
pub struct Args {
    ///🔌 hardware connection address
    #[structopt(short = "a", long = "address",  default_value = "tcp:host=192.168.66.59,port=6666")]
    address: String,
    ///🗁 sensor working directory
    #[structopt(short = "d", long = "dir",  default_value = "/home/sascha/.paw/mio/ndir")]
    dir: PathBuf,
    ///📢 subcommand to run.
    #[structopt(subcommand, about = "📢 subcommand to serve controller or start pipeline directly")]
    cmd:Cmd,
}

/// 🔧AIDE sensor argv methods
impl Args {
    /// Access the directory name.
    #[inline]
    pub fn command(&self) -> io::Result<()> {
               Ok(())
    }
    pub fn pid(&self) -> io::Result<PathBuf> {
        Ok(self.dir.join("pid"))
    }
}

#[paw::main]
fn main(args: Args) -> io::Result<()> {
    // println!("{}",Paint::blue(can::banner::NAME));
    femme::start(log::LevelFilter::Trace).unwrap();
    let pid = args.pid()?;
    ctrlc::set_handler(move || {
        if pid.is_file(){
            log::info!("pid file removed");
            fs::remove_file(&pid).expect("error - remove pid file");
        }
        process::abort();
    }).expect("Error setting Ctrl-C handler");
    match &args.cmd {
        Cmd::Average(average) => {
            let pid = args.pid()?;
            if pid.is_file() {
                log::warn!("sensor is busy pid {}",fs::read_to_string(args.pid()?)?);
                return Ok(())
            }
            fs::write(&pid,&format!("{}",process::id()))?;
            if let Err(e) = average.run(&args.dir,args.address.as_str()){
                eprintln!("{}", e)
            }
            fs::remove_file(&pid)?;
            // can::io::average_signal().await?;
        },
        Cmd::Setup(setup) => {
            setup.save(&args.dir)?;
            //
        },
        Cmd::Clean => {
            let pid =args.pid()?;
            if pid.is_file(){
                log::info!("remove pid file");
                let pstr = fs::read_to_string(&pid)?;
                fs::remove_file(&pid)?;
                process::Command::new("kill") .arg("-9").arg(pstr.as_str()).spawn().expect("kill edinburg process error");
            }
        }
    }

    Ok(())
}


