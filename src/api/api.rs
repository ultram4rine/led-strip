use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize)]
pub struct Color {
    pub white: f64,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

fn json_body() -> impl Filter<Extract = (Color,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn color_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    set_color()
}

pub fn set_color() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("color")
        .and(warp::post())
        .and(json_body())
        .and_then()
}
