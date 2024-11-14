use warp::Filter;
use crate::handlers::{get_fabric_orders, create_fabric_orders};
use crate::db::Db;

pub fn fabric_order_routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_orders = warp::path("orders")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(get_fabric_orders);

    let create_orders = warp::path("orders")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(create_fabric_orders);

    get_orders.or(create_orders)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
