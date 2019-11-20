#![feature(async_await,async_closure)]
use failure::Fallible;
use structopt::StructOpt;
// use serde_json::json;
use ansi_term::Colour;
use atty::Stream;
use std::io::{self, Read};

use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
// use runtime::time::Interval;
use std::time::{Duration, Instant};
// use futures::prelude::*;
use quicli::prelude::*;

use ndirs::check;

pub fn welcom()  {
    // use yansi::Paint;
    println!(r#" {:} "#,Colour::Blue.paint("   ██╗      █████╗ ██████╗    "));
    println!(r#" {:} "#,Colour::Blue.paint("   ██║     ██╔══██╗██╔══██╗   "));
    println!(r#" {:} "#,Colour::Blue.paint("   ██║     ███████║██████╔╝   "));
    println!(r#" {:} "#,Colour::Blue.paint("   ██║     ██╔══██║██╔══██╗   "));
    println!(r#" {:} "#,Colour::Blue.paint("   ███████╗██║  ██║██║  ██║   "));
    println!(r#" {:} "#,Colour::Blue.paint("   ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝   "));
    println!(r#" {:} "#,Colour::Blue.paint("ONLINE WATER QUALITY ANALYSERS"));
    println!(r#"      "#);
   


    println!(r#"     "#);
    println!(r#"      "#);
    println!(  " {:}  ",Colour::Blue.paint(format!("Number of logical cores is {}",num_cpus::get())));

    // println!(r#"  ENVIRONMENTAL MONITORING  "#);

}
use log::{LevelFilter,SetLoggerError,info};
use std::{
    // io::{self},
    // fs::{create_dir_all},
    path::{PathBuf},
};

// use std::{
    // io::{self},
    // fs::{create_dir_all},
    // path::{PathBuf},
// };

use async_log::{self,instrument, span};

pub fn setup_logger() {
    let logger = env_logger::Builder::new()
        .filter(None, log::LevelFilter::Trace)
        .build();

    async_log::Logger::wrap(logger, || /* get the task id here */ 0)
        .start(log::LevelFilter::Trace)
        .unwrap();
    span!("new level, depth={}", 1, {
        let x = "beep";
        info!("look at this value, x={}", x);

        span!("new level, depth={}", 2, {
            inner("boop");
        })
    });
}

#[instrument]
fn inner(y: &str) {
    info!("another nice value, y={}", y);
}





fn run_pipe() -> Fallible<()>{
    use dbus::Connection;
    use std::thread::sleep;
    use std::time::Duration;
    use ndirs::dbus::*;
    use ndirs::edinburgh::*;



    info!("Open dbus connection");
    info!("Create edinburg sensor");
    let mut ndir1 = Edinburgh::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
            println!("CMD:{}",input);
        let data = read_uart02()?;
        match ndir1.decode(data.as_slice()) {
            Ok(val) => println!("Res: {} - VAL:{}",val,ndir1.fsr),
            _ => (),
        }

    }
    Ok(())
}
/// The various kinds of commands that `wasm-pack` can execute.
#[derive(Debug, StructOpt)]
pub enum Command {
   ///  📈‍♀ run this command to confirm that your configuration is appropriately set up.
    #[structopt(name = "check", about = "check current sensor")]
    Check(check::Opt),
   ///  ⥄‍♀ run pipe mod.
    #[structopt(name = "pipe", about = "run in pipe mod")]
    Pipe,


}



/// 🧰  Edibnurgh sensor
#[derive(Debug, StructOpt)]
struct Cli {
    /// 🔧 Output JSON instead of human readable messages
    #[structopt(name = "json", long = "json")]
    json: bool,
    /// 📪  Address
    #[structopt(name = "address", long = "address", default_value = "tcp:host=192.168.100.133,port=6666")]
    address: String,
     /// The subcommand to run.
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    pub cmd: Command,
    /// ⏱  Message interval in milliseconds
    #[structopt(name = "interval", long = "interval", default_value = "1000")]
    interval: u64,
    /// ⦨  scale signal value
    #[structopt(name = "scale", long = "scale", default_value = "1.0")]
    scale: f64,
}



// async fn random_value(mut sig: RandomSignal) -> Fallible<f64>{
//     let val:f64 = (sig.next().unwrap() as f64) / 1000.0 ;

//     Ok(val)
// }

// #[runtime::main]
// async fn main() -> Fallible<()> {
//     // use ndirs::signal::RandomSignal;
//     let args = Cli::from_args();
//     match args.cmd {
//         Command::Check(subopt) => {check::run(subopt).await?;},
//     }
//     if atty::is(Stream::Stdout) {

//         println!("I'm a terminal");
//     } else {
//         println!("I'm not");
//     }
//     let start = Instant::now();
//     let mut interval = Interval::new(Duration::from_secs(2));
//     while let Some(now) = interval.next().await {
//         let elapsed = now - start;
//         println!("elapsed: {}s", elapsed.as_secs());
//     }
//         runtime::spawn(async move {
//         //  Delay::new(Duration::from_millis(args.interval))
//             // .map(|()| println!("printed after three seconds"))
//             // .wait()
//             // .unwrap();
//         dbg!("hello");
//     })
//     .await;
//     Ok(())
// }
fn main() -> CliResult {
    welcom();
    setup_logger();
    let args = Cli::from_args();
    if atty::is(Stream::Stdout) {

        println!("I'm a terminal");
    } else {
        println!("I'm not");
    }
    let res = match args.cmd {
        Command::Check(subopt) => {check::run(subopt)?;},
        Command::Pipe => run_pipe()?,
    };
    info!("result : {:?}",res);
    Ok(())
}
// fn main() -> Fallible<()> {
    // use ndirs::signal::RandomSignal;
    // fn main() {
    // env_logger::init();

    // setup_panic_hooks();

    // if let Err(e) = run() {
        // eprintln!("Error: {}", e);
        // for cause in e.iter_causes() {
            // eprintln!("Caused by: {}", cause);
        // }
        // ::std::process::exit(1);
    // }
// }
// }
