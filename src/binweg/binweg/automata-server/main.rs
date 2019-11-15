#![feature(async_await)]
#![feature(async_closure)]
#![allow(dead_code, unused_imports)]
#![allow(clippy::doc_markdown)]

// use structopt::*;
use automata;
use automata::Automata;
use async_std::{
    io,
    prelude::*,
    task,
    net::{TcpListener, TcpStream},
};
use log::{info,warn};
//::{
    // app,
    // templates,
    // routes,
// };
//  use automata::api::{
//     uv as analyzer,
    // app,
// };

// use automata::mio;


// use tide::{Error};
// use crate::{
    // config::{
        // ServerConfig,
    // },
// };

// use runtime;
// use tide::{self, App, Context, EndpointResult, Error};
use tide;



// impl automataError {
//     pub fn response_500(self) -> Error {
//         let resp = http::Response::builder()
//                 .status(500)
//                 .body("response".to_owned().into())
//                 .unwrap();
//         Error::from(resp)
//     }
// }



// async fn collect(_cx: Context<()>) -> EndpointResult<http::Response<Body>> {
//     let mut buffer = BytesMut::with_capacity(16_384);

//     automata::data::cpu::cpu(&mut buffer).await;
//     automata::data::host::host(&mut buffer).await;
//     automata::data::disk::disk(&mut buffer).await;
//     automata::data::memory::memory(&mut buffer).await;

//     let resp = http::Response::builder()
//         .status(http::status::StatusCode::OK)
//         .body(Body::from(buffer))
//         .unwrap();
//     Ok(resp)
// }
pub struct StateLocal{}

async fn server(address: &str) -> io::Result<()> {
// let file_path = automata::MIO.path().join("my-temporary-note.txt");
    // let mut file = File::create(file_path)?;
    // writeln!(file, "Brian was here. Briefly.")?;
    // let repo = monitor::new_uv().await;
    let mut app      = tide::App::with_state(StateLocal{});
    app.middleware(tide::middleware::RequestLogger::new());
    info!("🌩️   starting broker");
    warn!("⚠️   setup warning test");
    // app.middleware(tide::middleware::RequestLogger::new());
    // app.at("/").get(automata::templates::example_index);
    // app = routes::setup(app);
    app.run(address)?;
    Ok(())
}

fn main()  -> io::Result<()>{
    use log::{info,warn};
    use log::LevelFilter;
    // use log4rs::append::console::ConsoleAppender;
    // use log4rs::config::{Appender, Config, Root};
    use tempfile::tempdir;

    use yansi::Paint;
    // use std::fs;
    use std::fs::File;
    use std::io::{ Write};

    // let automata = automata::new("/automata");
    // let welcome = automata::Asset::get("welcome.md").unwrap();
    // let welcom = fs::read_to_string()
    // println!(r#" {:#?} "#,Paint::blue( std::str::from_utf8(welcome.as_ref())));
    // println!(r#" {:#?} "#,Paint::blue("ONLINE WATER QUALITY ANALYSIS ENVIRONMENTAL MONITORING"));
    automata::config::setup();
    automata::hello();
    // let server_config = automata.config().server();

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    // let _handle = log4rs::init_config(config).unwrap();
     //  let template_dir = format!("{}/examples/templates/*", env!("CARGO_MANIFEST_DIR"));
    // TODO: load configuration.
    // let severConfig = automata::config::ServerConfig::default();
    //  if !Path::new("info.toml").exists() {
        // println!(
            // "{} must be run from the rustlings directory",
            // std::env::current_exe().unwrap().to_str().unwrap()
        // );
        // println!("Try `cd rustlings/`!");
        // std::process::exit(1);
    // }

    // let toml_str = &fs::read_to_string("info.toml").unwrap();
    // Create a directory inside of `std::env::temp_dir()`.
    // let dir = tempdir()?;
    // println!("Pub mio temp directory {:?}",automata::MIO.path());
    task::block_on(server("127.0.0.1:8080"))

    // analyzer::store::setup(".").await?;
    //
    // automata::server::`
}
