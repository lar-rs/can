#![feature(async_await)]

// use structopt::*;
use tide;
use wqm_can::{
  analog,
  digital,
  doppelmotor,
};

use wqm_can::State;
use runtime;
use failure::{Fallible};
use std::fmt::Write;
use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;
// use failure::{Fallible, ResultExt};



async fn analog1_info(_cx: Context<State>) -> EndpointResult<String> {
    let mut info = String::new();
    write!(&mut info, "Analog01 info").unwrap(); // uses fmt::Write::write_fmt
    Ok(info)
}

async fn analog1_get_input01(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_input01(0x2)))
}
async fn analog1_get_input02(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_input02(0x2)))
}
async fn analog1_get_input03(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_input03(0x2)))
}
async fn analog1_get_input04(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_input04(0x2)))
}
async fn analog1_get_input05(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_input05(0x2)))
}
async fn analog1_get_out(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_out(0x2)))
}
async fn analog1_set_out(cx: Context<State>) -> EndpointResult<()>  {
    let value:u16 = cx.body_json().await.client_err()?;
    analog::set_out(0x2,value);
    Ok(())
}
async fn analog1_get_temp01(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_temp01(0x2)))
}
async fn analog1_get_temp02(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_temp02(0x2)))
}
async fn analog1_get_temp03(cx: Context<State>) -> EndpointResult {
    Ok(response::json(analog::get_temp03(0x2)))
}






async fn can0_info(_cx: Context<State>) -> EndpointResult<String> {
    let mut info = String::new();
    write!(&mut info, "can0 info").unwrap(); // uses fmt::Write::write_fmt
    Ok(info)

}

/// An example of how to run a Tide service on top of `runtime`, this also shows the pieces
/// necessary if you wish to run a service on some other executor/IO source.

#[runtime::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use log::LevelFilter;
    use log4rs::append::console::ConsoleAppender;
    use log4rs::config::{Appender, Config, Root};

    // First, we create a simple hello world application
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    // let repo = monitor::new_uv().await;
    let mut app = App::with_state(State::default());

    // let mut app = tide::App::new(());
    app.middleware(tide::middleware::RequestLogger::new());
    app.at("/can0").nest(|api| {
        api.at("/analog/info").get(analog1_info);
        api.at("/analog/in01").get(analog1_get_input01);
        api.at("/analog/in02").get(analog1_get_input02);
        api.at("/analog/in03").get(analog1_get_input03);
        api.at("/analog/in04").get(analog1_get_input04);
        api.at("/analog/in05").get(analog1_get_input05);
        api.at("/analog/out").get(analog1_get_out).post(analog1_set_out);
        api.at("/analog/:id/temp01").get(analog1_get_temp01);
        api.at("/analog/:id/temp02").get(analog1_get_temp02);
        api.at("/analog/:id/temp03").get(analog1_get_temp03);
        // api.at("/analog/:id/uart01").get(analog::get_uart01).post(analog::set_uart01);
        // api.at("/analog/:id/baut01").post(analog::setup_uart02);
        // api.at("/analog/:id/uart02").get(analog::get_uart02).post(analog::set_uart02);
        // api.at("/analog/:id/baut02").post(analog::setup_uart02);
        // api.at("/digital/:id").get(digital::info);
        // api.at("/digital/:id/input").get(digital::get_input);
        // api.at("/digital/:id/output").get(digital::get_output).post(digital::set_output);
        // api.at("/doppelmotor/:id/uart01").get(doppelmotor::get_uart01).post(doppelmotor::set_uart01);
        // api.at("/doppelmotor/:id/baut01").post(doppelmotor::setup_uart02);
        // api.at("/doppelmotor/:id/uart02").get(doppelmotor::get_uart02).post(doppelmotor::set_uart02);
        // api.at("/doppelmotor/:id/baut02").post(doppelmotor::setup_uart02);
    //   api.at("/analog/:id/in01").get(analog::get_input01);
    });
    app.run("127.0.0.1:8000").unwrap();

    // Instead of using `App::run` to start the application, which implicitly uses a default
    // http-service server, we need to configure a custom server with the executor and IO source we
    // want it to use and then run the Tide service on it.

    // Turn the `tide::App` into a generic `http_service::HttpService`
    // let http_service = app.into_http_service();

    // Build an `http_service_hyper::Server` using runtime's `TcpListener` and `Spawn` instances
    // instead of hyper's defaults.
    // let mut listener = runtime::net::TcpListener::bind("127.0.0.1:8000")?;
    // let server = http_service_hyper::Server::builder(listener.incoming())
        // .with_spawner(runtime::task::Spawner::new());

    // Serve the Tide service on the configured server, and wait for it to complete
    // server.serve(http_service).await?;

    Ok(())
}
