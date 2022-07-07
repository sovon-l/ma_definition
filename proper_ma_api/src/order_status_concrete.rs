#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OrderStatusConcrete {
    Submitting = 0x0_u8, 
    Submitted = 0x1_u8, 
    Cancelling = 0x2_u8, 
    Finished = 0x3_u8, 
    NullVal = 0xff_u8, 
}
impl Default for OrderStatusConcrete {
    #[inline]
    fn default() -> Self { OrderStatusConcrete::NullVal }
}
impl From<u8> for OrderStatusConcrete {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::Submitting, 
            0x1_u8 => Self::Submitted, 
            0x2_u8 => Self::Cancelling, 
            0x3_u8 => Self::Finished, 
            _ => Self::NullVal,
        }
    }
}
