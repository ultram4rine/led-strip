#![deny(warnings)]
mod api;
mod controller;
mod led;

use crate::api::api::color_routes;

#[tokio::main]
async fn main() {
    warp::serve(color_routes).run(([127, 0, 0, 1], 3030)).await;
}
