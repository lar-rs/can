use automata::mio;
use tide::{
    error::{ StringError, ResultExt },
    response, App, Context, EndpointResult,
    querystring::ContextExt
};

use super::app::State;
/// airflow_input
async fn airflow_input(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_airflow_input().await.unwrap()))
}

async fn airflow_output(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_airflow_input().await.unwrap()))
}

async fn humidity(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_humidity().await.unwrap()))
}

async fn pressure(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_humidity().await.unwrap()))
}

async fn sample_pump(mut cx: Context<State>) -> EndpointResult<()> {
    let start = cx.body_json().await.client_err()?;
    mio::sample_pump(start).await.unwrap();
    Ok(())
}

async fn stream1_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":open").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream2_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream3_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream4_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream5_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn stream6_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::stream1_valve(state).await.unwrap();
    Ok(())
}
async fn zeroflow_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::zeroflow_valve(state).await.unwrap();
    Ok(())
}
async fn tic_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::tic_valve(state).await.unwrap();
    Ok(())
}
async fn calibration_valve(cx: Context<State>) -> EndpointResult<()> {
    let state:bool = cx.param(":state").client_err()?;
    mio::calibration_valve(state).await.unwrap();
    Ok(())
}
async fn get_ndir1(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_ndir1_value().await.unwrap()))
}
async fn get_ndir2(_cx: Context<State>) -> EndpointResult {
    Ok(response::json(mio::get_ndir2_value().await.unwrap()))
}

pub fn setup_routes(mut app: App<State>) -> App<State> {

    app.at("/uv").nest(|api| {
        api.at("/airflow/in").get(airflow_input);
        api.at("/airflow/out").get(airflow_output);
        api.at("/humidity").get(humidity);
        api.at("/pressure").get(pressure);
        api.at("/sample/:start").post(sample_pump);
        api.at("/valve/1/:open").post(stream1_valve);
        api.at("/valve/2/:open").post(stream2_valve);
        api.at("/valve/3/:open").post(stream3_valve);
        api.at("/valve/4/:open").post(stream4_valve);
        api.at("/valve/5/:open").post(stream5_valve);
        api.at("/valve/6/:open").post(stream6_valve);
        api.at("/valve/zeroflow/:state").post(zeroflow_valve);
        api.at("/valve/tic/:state").post(tic_valve);
        api.at("/valve/calibration/:state").post(calibration_valve);
        api.at("/ndir1").get(get_ndir1);
        api.at("/ndir2").get(get_ndir2);
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
    #![feature(async_await)]
    use http_service_mock::make_server;

    use http_service;
    use super::*;
    use tide::{
        Server,
        error::{ StringError, ResultExt },
        response, App, Context, EndpointResult,
        querystring::ContextExt
    };

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
