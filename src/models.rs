use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricOrder {
    pub srno: u32,
    pub fabric_status: String,
    pub fabric_status_notes: String,
    pub order_no: String,
    pub order_date: String,
    pub order_since_days: u32,
    pub salestrip: String,
    pub payment_type: String,
    pub cc_pass_date: String,
    pub cc_pass_days: u32,
}
