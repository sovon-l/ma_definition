#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Time_in_force {
    GoodTill = 0x0_u8, 
    IOC = 0x1_u8, 
    FOK = 0x2_u8, 
    NullVal = 0xff_u8, 
}
impl Default for Time_in_force {
    #[inline]
    fn default() -> Self { Time_in_force::NullVal }
}
impl From<u8> for Time_in_force {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::GoodTill, 
            0x1_u8 => Self::IOC, 
            0x2_u8 => Self::FOK, 
            _ => Self::NullVal,
        }
    }
}
