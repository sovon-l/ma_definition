#[derive(Debug, Clone)]
pub struct Depth {
    pub price: rust_decimal::Decimal,
    pub size: rust_decimal::Decimal,
}

#[derive(Debug, Clone)]
pub struct Quotes {
    pub symbol: crate::structs::market::instrument::Instrument,
    pub market_timestamp: u64,
    pub timestamp: Option<u64>, // ns
    pub is_snapshot: bool,
    pub is_l1: bool,
    pub depths: Vec<Depth>,
}