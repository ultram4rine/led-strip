#![deny(warnings)]
#![allow(dead_code)]
mod controller;
mod led;

use crate::controller::controller::Controller;
use crate::led::led::LED;
use std::convert::Infallible;
use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    let controller = Controller::new();

    let routes = enable(controller.clone())
        .or(disable(controller.clone()))
        .or(color(controller.clone()))
        .with(warp::cors().allow_any_origin());

    warp::serve(warp::fs::dir("ui/public").or(routes))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn enable(
    controller: Controller,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("enable")
        .and(warp::post())
        .and(warp::any().map(move || controller.clone()))
        .and_then(enable_led)
}

fn disable(
    controller: Controller,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("disable")
        .and(warp::post())
        .and(warp::any().map(move || controller.clone()))
        .and_then(disable_led)
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

async fn enable_led(mut controller: Controller) -> Result<impl warp::Reply, Infallible> {
    match controller.enable().await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn disable_led(mut controller: Controller) -> Result<impl warp::Reply, Infallible> {
    match controller.disable().await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn apply_color(led: LED, mut controller: Controller) -> Result<impl warp::Reply, Infallible> {
    match controller.apply(led).await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(e) => {
            println!("{:?}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
}
