#![deny(warnings)]
mod api;
mod controller;
mod led;

#[tokio::main]
async fn main() {
    let routes = api::api::color_routes();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
