#![deny(warnings)]
#![allow(dead_code)]
mod controller;
mod led;

use crate::controller::controller::Controller;
use crate::led::led::LED;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter};

#[derive(Clone, Deserialize, Serialize)]
struct Credentials {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let controller = Arc::new(Mutex::new(Controller::new()));

    let admin_user = env::var("LED_USER").unwrap();
    let admin_pass = env::var("LED_PASS").unwrap();

    let api = api(
        Credentials {
            username: admin_user,
            password: admin_pass,
        },
        controller,
    )
    .with(warp::cors().allow_any_origin());

    warp::serve(warp::fs::dir("ui/public").or(api))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn api(
    admin: Credentials,
    controller: Arc<Mutex<Controller>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    login(admin)
        .or(status(controller.clone()))
        .or(enable(controller.clone()))
        .or(disable(controller.clone()))
        .or(set_color(controller))
}

fn with_controller(
    controller: Arc<Mutex<Controller>>,
) -> impl Filter<Extract = (Arc<Mutex<Controller>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || controller.clone())
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

fn status(
    controller: Arc<Mutex<Controller>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("status")
        .and(warp::get())
        .and(with_controller(controller))
        .and_then(get_status)
}

fn enable(
    controller: Arc<Mutex<Controller>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("enable")
        .and(warp::post())
        .and(with_controller(controller))
        .and_then(enable_led)
}

fn disable(
    controller: Arc<Mutex<Controller>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("disable")
        .and(warp::post())
        .and(with_controller(controller))
        .and_then(disable_led)
}

fn set_color(
    controller: Arc<Mutex<Controller>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("color")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_controller(controller))
        .and_then(apply_color)
}

async fn auth(user: Credentials, admin: Credentials) -> Result<impl warp::Reply, Infallible> {
    match user.username == admin.username && user.password == admin.password {
        true => return Ok(StatusCode::OK),
        false => return Ok(StatusCode::UNAUTHORIZED),
    }
}

async fn get_status(controller: Arc<Mutex<Controller>>) -> Result<impl warp::Reply, Infallible> {
    let c = controller.lock().await;
    let state = c.clone();
    Ok(warp::reply::json(&state))
}

async fn enable_led(controller: Arc<Mutex<Controller>>) -> Result<impl warp::Reply, Infallible> {
    let mut c = controller.lock().await;
    match c.enable().await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn disable_led(controller: Arc<Mutex<Controller>>) -> Result<impl warp::Reply, Infallible> {
    let mut c = controller.lock().await;
    match c.disable().await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn apply_color(
    led: LED,
    controller: Arc<Mutex<Controller>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut c = controller.lock().await;
    match c.apply(led).await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(e) => {
            println!("{:?}", e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
}
