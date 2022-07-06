#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderType {
    limit = 0x0_u8, 
    market = 0x1_u8, 
    NullVal = 0xff_u8, 
}
impl Default for OrderType {
    #[inline]
    fn default() -> Self { OrderType::NullVal }
}
impl From<u8> for OrderType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::limit, 
            0x1_u8 => Self::market, 
            _ => Self::NullVal,
        }
    }
}
