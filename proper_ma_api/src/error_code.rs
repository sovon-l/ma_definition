#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ErrorCode {
    InsufficientFund = 0x0_u8, 
    FailPostOnly = 0x1_u8, 
    Timeout = 0x2_u8, 
    GatewayTooManyRequests = 0x3_u8, 
    ExchangeTooManyRequests = 0x4_u8, 
    ExchangeConnectionError = 0x5_u8, 
    UnexpectedExchangeError = 0x6_u8, 
    UnexpectedGatewayError = 0x7_u8, 
    NullVal = 0xff_u8, 
}
impl Default for ErrorCode {
    #[inline]
    fn default() -> Self { ErrorCode::NullVal }
}
impl From<u8> for ErrorCode {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::InsufficientFund, 
            0x1_u8 => Self::FailPostOnly, 
            0x2_u8 => Self::Timeout, 
            0x3_u8 => Self::GatewayTooManyRequests, 
            0x4_u8 => Self::ExchangeTooManyRequests, 
            0x5_u8 => Self::ExchangeConnectionError, 
            0x6_u8 => Self::UnexpectedExchangeError, 
            0x7_u8 => Self::UnexpectedGatewayError, 
            _ => Self::NullVal,
        }
    }
}
