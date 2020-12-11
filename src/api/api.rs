use crate::led::led::LED;
use warp::{http, Filter};

fn json_body() -> impl Filter<Extract = (LED,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn color_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    set_color()
}

async fn print(led: LED) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{}", led.white);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

pub fn set_color() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("color")
        .and(warp::post())
        .and(json_body())
        .and_then(print)
}
