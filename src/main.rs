#![deny(warnings)]
#![allow(dead_code)]
mod controller;
mod led;

use crate::controller::controller::Controller;
use crate::led::led::LED;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use warp::{http::StatusCode, Filter};

#[derive(Clone, Deserialize, Serialize)]
struct Credentials {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let controller = Controller::new();

    let admin_user = env::var("LED_USER").unwrap();
    let admin_pass = env::var("LED_PASS").unwrap();

    let routes = login(Credentials {
        username: admin_user,
        password: admin_pass,
    })
    .or(enable(controller.clone()))
    .or(disable(controller.clone()))
    .or(set_color(controller.clone()))
    .with(warp::cors().allow_any_origin());

    warp::serve(warp::fs::dir("ui/public").or(routes))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn login(
    admin: Credentials,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || admin.clone()))
        .and_then(auth)
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

fn set_color(
    controller: Controller,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("color")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || controller.clone()))
        .and_then(apply_color)
}

async fn auth(user: Credentials, admin: Credentials) -> Result<impl warp::Reply, Infallible> {
    match user.username == admin.username && user.password == admin.password {
        true => return Ok(StatusCode::OK),
        false => return Ok(StatusCode::UNAUTHORIZED),
    }
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
    controller.led = led;

    match controller.apply().await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(e) => {
            println!("{:?}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
}
