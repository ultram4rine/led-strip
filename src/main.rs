#![deny(warnings)]
mod controller;
mod led;

use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
