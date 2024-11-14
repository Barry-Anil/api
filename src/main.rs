mod models;
mod handlers;
mod routes;
mod db;

use warp::Filter;

#[tokio::main]
async fn main() {
    let db = db::init_db();

    let api = routes::fabric_order_routes(db);

    println!("Server running on http://127.0.0.1:3030");
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
