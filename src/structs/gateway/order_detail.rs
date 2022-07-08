use rust_decimal::Decimal;

// TODO: implement contingent orders / price algo orders

#[derive(Debug, Clone)]
pub enum OrderDetail {
    SimpleOrder(SimpleOrderDetail),
    // OCO(SimpleOrderDetail, SimpleOrderDetail),
    // OTO(SimpleOrderDetail, SimpleOrderDetail),
    // TODO: Trailing
    // TODO: Adaptive
}

#[derive(Debug, Clone)]
pub enum SimpleOrderDetail {
    BasicOrder(BasicOrderDetail),
    TriggerOrder {
        is_stop_loss: bool, // false: is take profit
        trigger: Decimal,
        execute: BasicOrderDetail,
    },
}

#[derive(Debug, Clone)]
pub enum BasicOrderDetail {
    Limit {
        instrument: crate::structs::market::instrument::Instrument,
        price: Decimal,
        amount: Decimal,
        time_in_force: TimeInForce,
        post_only: bool,
        reduce_only: bool,
    },
    Market {
        instrument: crate::structs::market::instrument::Instrument,
        amount: Decimal,
        reduce_only: bool,
    },
}

#[derive(Debug, Clone)]
pub enum TimeInForce {
    GoodTill(Option<u64>), // None: GTC; with value: auto cancel after (u64)us
    ImmediateOrCancel,
    FillOrKill,
}