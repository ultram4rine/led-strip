use crate::controller::controller::Controller;
use crate::led::led::LED;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;

#[derive(Clone, Deserialize, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub async fn auth(user: Credentials, admin: Credentials) -> Result<impl warp::Reply, Infallible> {
    match user.username == admin.username && user.password == admin.password {
        true => return Ok(StatusCode::OK),
        false => return Ok(StatusCode::UNAUTHORIZED),
    }
}

pub async fn get_status(
    controller: Arc<Mutex<Controller>>,
) -> Result<impl warp::Reply, Infallible> {
    let c = controller.lock().await;
    let state = c.clone();
    Ok(warp::reply::json(&state))
}

pub async fn enable_led(
    controller: Arc<Mutex<Controller>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut c = controller.lock().await;
    match c.enable().await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn disable_led(
    controller: Arc<Mutex<Controller>>,
) -> Result<impl warp::Reply, Infallible> {
    let mut c = controller.lock().await;
    match c.disable().await {
        Ok(()) => return Ok(StatusCode::OK),
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn apply_color(
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
