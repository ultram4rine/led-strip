#![deny(warnings)]
#![allow(dead_code)]
mod controller;
mod led;

extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use crate::controller::controller::Controller;
use crate::led::led::LED;
use std::convert::Infallible;
use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    let controller = Controller::new();

    let routes = color(controller.clone()).with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn color(
    controller: Controller,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("color")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || controller.clone()))
        .and_then(apply_color)
}

async fn apply_color(led: LED, mut controller: Controller) -> Result<impl warp::Reply, Infallible> {
    match controller.apply(led).await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    };
}
