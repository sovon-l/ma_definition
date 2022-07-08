#[derive(Debug, Clone)]
pub struct Trades {
    pub symbol: crate::structs::market::instrument::Instrument,
    pub market_timestamp: u64,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub price: rust_decimal::Decimal,
    pub size: rust_decimal::Decimal,
    pub timestamp: u64,
    // tradeId: u32,
}

