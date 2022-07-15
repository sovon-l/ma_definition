use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderProgress {
    pub filled_amount: Decimal,
    pub paid_amount: Decimal,
    pub commission: Decimal,
    pub order_status: OrderStatus,
    pub last_update: u64,
    pub last_exchange_update: Option<u64>,
}

impl Default for OrderProgress {
    fn default() -> Self {
        OrderProgress {
            filled_amount: rust_decimal::Decimal::ZERO,
            paid_amount: rust_decimal::Decimal::ZERO,
            commission: rust_decimal::Decimal::ZERO,
            order_status: OrderStatus::Submitting,
            last_update: chrono::offset::Utc::now()
                .timestamp_nanos()
                .try_into()
                .unwrap(),
            last_exchange_update: None,
        }
    }
}

impl OrderProgress {
    pub fn update(&mut self) {
        self.last_update = chrono::offset::Utc::now()
            .timestamp_nanos()
            .try_into()
            .unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Submitting,
    Submitted,
    Cancelling,
    Finished(Option<ExecuteErrorCode>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteErrorCode {
    InsufficientFund,
    FailPostOnly,
    Timeout,
    GatewayTooManyRequests,

    ExchangeTooManyRequests,
    ExchangeConnectionError,
    UnexpectedExchangeError,
    UnexpectedGatewayError,
}

impl std::convert::From<ExecuteErrorCode> for OrderStatus {
    fn from(e: ExecuteErrorCode) -> Self {
        OrderStatus::Finished(Some(e))
    }
}
