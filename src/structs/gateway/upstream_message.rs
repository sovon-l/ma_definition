pub enum Message {
    Place(super::order::GatewayOrder),
    Cancel(OrderId),
}

pub enum OrderId {
    Upstream(u64),
    Local(u64),
}
