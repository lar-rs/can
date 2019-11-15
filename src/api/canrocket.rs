#![feature(proc_macro_hygiene, decl_macro, try_trait)]

use rocket::{get, routes};

#[cfg(test)]
mod testing;

use log::info;

use lazy_static::lazy_static;
use prometheus::{opts, IntCounterVec};
use rocket_prometheus::PrometheusMetrics;
use wqm_can::*;
lazy_static! {
    static ref NAME_COUNTER: IntCounterVec =
        IntCounterVec::new(opts!("name_counter", "Count of names"), &["name"]).unwrap();
}
#[get("/info")]
fn analog1_info(_cx: Context<State>) -> EndpointResult<String> {
    let mut info = String::new();
    write!(&mut info, "Analog01 info").unwrap(); // uses fmt::Write::write_fmt
    Ok(info)
}
fn analog1_get_input01(hit_count: State<Canbus>) -> EndpointResult {
    Ok(response::json(analog::get_input01(0x2)))
}
// async fn analog1_get_input02(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_input02(0x2)))
// }
// async fn analog1_get_input03(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_input03(0x2)))
// }
// async fn analog1_get_input04(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_input04(0x2)))
// }
// async fn analog1_get_input05(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_input05(0x2)))
// }
// async fn analog1_get_out(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_out(0x2)))
// }
// async fn analog1_set_out(cx: Context<State>) -> EndpointResult<()>  {
//     let value:u16 = cx.body_json().await.client_err()?;
//     analog::set_out(0x2,value);
//     Ok(())
// }
// async fn analog1_get_temp01(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_temp01(0x2)))
// }
// async fn analog1_get_temp02(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_temp02(0x2)))
// }
// async fn analog1_get_temp03(cx: Context<State>) -> EndpointResult {
//     Ok(response::json(analog::get_temp03(0x2)))
// }


fn count(hit_count: State<'_, HitCount>) -> String {
    hit_count.0.load(Ordering::Relaxed).to_string()
}


fn main() {
    info!("✨ May your hopes and dreams become reality ✨");


    rocket::ignite()
        .attach(prometheus.clone())
        .mount("analog1/in01",m)
        .launch();

    // config::load_config();

    // rocket
    //     .mount("/github", routes![service::github_event])
    //     .mount("/gitlab", routes![service::gitlab_event])
    //     .mount("/", routes![service::check])
    //     .register(catchers![
    //         errors::not_found,
    //         errors::internal_server_error,
    //         errors::unprocessable_entity
    //     ])
    //     .launch();
}
