// use crate::mio;
use tide::{
    error::{ ResultExt },
    response, App, Context, EndpointResult,
};

use crate::{
    store::{
        self,
        stream::Stream,
    },
    mio,
};

use super::app::State;
// use super::sensor;
// use super::measure::*;




/// Device get status
pub async fn response_device(_cx: Context<State>) -> EndpointResult {
    let device = store::device::read().await.unwrap();
    Ok(response::json(device))
}
/// Device get serial number
// pub async fn get_device_serial(_cx: Context<State>) -> EndpointResult {
    // Ok(response::json(store::get_local().await.unwrap().get_serial()))
// }
/// Divice set serial number
// pub async fn device_(mut cx:Context<State>) -> EndpointResult<()> {
    // let device = cx.body_json().await.client_err()?;
    // store::et_local().await.unwrap().set_serial(serial);
    // Ok(())
// }
pub async fn response_streams(_cx: Context<State>) -> EndpointResult {
    // let app = cx.state();
    let list = store::stream::read_all().await.unwrap();
    Ok(response::json (list))
}

pub async fn response_stream(cx: Context<State>) -> EndpointResult {
    let stream:u64 = cx.param("stream").client_err()?;
    let stream = store::stream::read(stream).await.unwrap();
    Ok(response::json(stream))
}

pub async fn post_stream(mut cx: Context<State>) -> EndpointResult<()> {
    let stream:Stream = cx.body_json().await.client_err()?;
    store::stream::save(&stream).await.unwrap();
    Ok(())
}


/// Channel settings api
pub async fn response_stream_channels(cx: Context<State>) -> EndpointResult {
    let id:u64 = cx.param("stream").client_err()?;

    // let app = cx.state();
    let channels = store::stream::read_channels(id).await.unwrap();
    Ok(response::json(channels))
}
// async fn response_stream_channel(cx: Context<State>) -> EndpointResult {
//     let stream:u64 = cx.param("stream").client_err()?;
//     let channel:u64 = cx.param("channel").client_err()?;
//     let channel = store::get_channel_from_ids(stream,channel).await.unwrap();
//     Ok(list)
// set_stream
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }
// async fn post_stream_channel(cx: Context<State>) -> EndpointResult {
    // let stream:u64 = cx.param("stream").client_err()?;
    // let channel:u64 = cx.param("channel").client_err()?;

/// set_stream
// pub async fn set_stream_name(_cx: Context<State>) -> EndpointResult<()> {
    // Ok(())
// }
pub async fn response_rules(_cx: Context<State>) -> EndpointResult {
    let rules = store::rule::read_all().await.unwrap();
    Ok(response::json(rules))
}
/// Get rule from id
///
pub async fn response_rule(cx:Context<State>) -> EndpointResult {
    let rule:u64 = cx.param("rule").client_err()?;
    let rule = store::rule::read(rule).await.unwrap();
    Ok(response::json (rule))
}


/// Set rule
pub async fn post_rule(mut cx: Context<State>) -> EndpointResult<()> {
    let rule = cx.body_json().await.client_err()?;
    store::rule::save(rule).await.unwrap();
    Ok(())
}

/// Endpoint:
///   POST /api/uv/sample/start
async fn handle_start_sample(_cx: Context<State>) -> EndpointResult<()> {
    mio::uv::sample_pump(true).await.unwrap();
    Ok(())
}
/// Endpoint:
///   POST /api/uv/sample/start
async fn handle_stop_sample(_cx: Context<State>) -> EndpointResult<()> {
    mio::uv::sample_pump(false).await.unwrap();
    Ok(())
}
/// Endpoint:
///   POST /api/uv/sample/start
async fn handle_open_sample_valve(cx: Context<State>) -> EndpointResult<()> {
    let num = cx.param(":num").client_err()?;
    mio::uv::open_sample_valve(num).await.unwrap();
    Ok(())
}
async fn handle_close_sample_valve(cx: Context<State>) -> EndpointResult<()> {
    let num= cx.param(":num").client_err()?;
    mio::uv::open_sample_valve(num).await.unwrap();
    Ok(())
}
async fn open_zeroflow_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::open_zeroflow_valve().await.unwrap();
    Ok(())
}
async fn close_zeroflow_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::close_zeroflow_valve().await.unwrap();
    Ok(())
}
async fn open_tic_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::open_tic_valve().await.unwrap();
    Ok(())
}
async fn close_tic_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::close_tic_valve().await.unwrap();
    Ok(())
}
async fn open_calibration_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::open_calibration_valve().await.unwrap();
    Ok(())
}
async fn close_calibration_valve(_cx: Context<State>) -> EndpointResult<()> {
    // let state:bool = cx.param(":state").client_err()?;
    mio::uv::close_calibration_valve().await.unwrap();
    Ok(())
}
async fn response_ndir1_value(_cx: Context<State>) -> EndpointResult {
    let val =  0.0;// mio::sensor::get_ndir1_value().await.unwrap();
    Ok(response::json(val))
}
async fn response_ndir2_value(_cx: Context<State>) -> EndpointResult {
    let val = 0.0;//mio::sensor::get_ndir2_value().await.unwrap();
    Ok(response::json(val))
}


/// Uv io state return all hardware parameter.
async fn handle_state(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(0))
}


async fn get_airflow_input(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::airflow::get_airflow_input().await.unwrap()))
}

/// get airflow current value on in
async fn get_airflow_output(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::airflow::get_airflow_input().await.unwrap()))
}

async fn get_humidity(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::humidity::get_humidity().await.unwrap()))
}

async fn get_pressure(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::humidity::get_humidity().await.unwrap()))
}

async fn handle_can_frame(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::humidity::get_humidity().await.unwrap()))
}

pub fn setup(mut app: App<State>) -> App<State> {

    app.at("/api/").nest(|api| {
        api.at("/state").get(handle_state);
        api.at("/gp1/start").post(handle_start_sample);
        api.at("/gp1/stop").post(handle_stop_sample);
        api.at("/sample/:num/open").post(handle_open_sample_valve);
        api.at("/sample/close").post(handle_close_sample_valve);
        api.at("/valve/zeroflow/open").post(open_zeroflow_valve);
        api.at("/valve/zeroflow/close").post(close_zeroflow_valve);
        api.at("/valve/tic/open").post(open_tic_valve);
        api.at("/valve/tic/close").post(close_tic_valve);
        api.at("/valve/calibration/open").post(open_calibration_valve);
        api.at("/valve/calibration/close").post(close_calibration_valve);
        api.at("/ndir1").get(response_ndir1_value);
        api.at("/ndir2").get(response_ndir2_value);
        api.at("/airflow/input").get(get_airflow_input);
        api.at("/airflow/output").get(get_airflow_output);
        api.at("/humidity").get(get_humidity);
        api.at("/pressure").get(get_pressure);
        api.at("/device").get(response_device);
        api.at("/streams").get(response_streams).post(post_stream);
        api.at("/:stream").get(response_stream);
        api.at("/:stream/channels").get(response_stream_channels);
        // api.at("/:stream/:channel").get(response_stream_channel).post(post_stream_channel);
        api.at("/rules").get(response_rules);
        api.at("/:rule").get(response_rule).post(post_rule);
        api.at("/can/frame").post(handle_can_frame);
        // api.at("/lamp").get(get_signal);
        // api.at("/lamp/on").get(get_signal);
        // api.at("/lamp/off").get(get_signal);
        // api.at("/streams").get(stream)
        // api.at("/info").get(device::get_info);
    });
    app
}

#[cfg(test)]
mod tests {
    // #![feature(async_await)]
    // use http_service_mock::make_server;

    // use http_service;
    // use super::*;
    // use tide::{
    //     Server,
    //     error::{ StringError, ResultExt },
    //     response, App, Context, EndpointResult,
    //     querystring::ContextExt
    // };

    #[test]
    fn uvtest() {
        // let state = State::new();
        // let mut app      = tide::App::with_state(state);
        // app = setup_routes(app);
        // let mut server = make_server(app.into_http_service()).unwrap();

        // let req = http::Request::get("/add_one/3")
        //     .body(Body::empty())
        //     .unwrap();
        // let res = server.simulate(req).unwrap();
        // assert_eq!(res.status(), 200);
        // let body = block_on(res.into_body().into_vec()).unwrap();
        // assert_eq!(&*body, &*b"4");
    }

}
