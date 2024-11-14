use crate::models::FabricOrder;
use crate::db::Db;
use warp::http::StatusCode;
use warp::Reply;

pub async fn get_fabric_orders(db: Db) -> Result<impl Reply, warp::Rejection> {
    let db = db.lock().unwrap();
    let orders = db.clone();
    Ok(warp::reply::json(&orders))
}

pub async fn create_fabric_orders(orders: Vec<FabricOrder>, db: Db) -> Result<impl Reply, warp::Rejection> {
    let mut db = db.lock().unwrap();
    for order in orders {
        db.push(order);
    }
    Ok(warp::reply::with_status("Orders added", StatusCode::CREATED))
}
