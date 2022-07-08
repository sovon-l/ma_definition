
#[derive(Debug, Clone)]
pub struct GatewayOrder {
    pub upstream_order_id: u64,
    pub timestamp: u64,
    pub acc_id: super::account::Account,
    pub order_detail: super::order_detail::OrderDetail,
}
