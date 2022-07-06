#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Order_options(pub u8);
impl Order_options {
    #[inline]
    pub fn new(value: u8) -> Self {
        Order_options(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_reduce_only(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_reduce_only(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }
}
impl core::fmt::Debug for Order_options {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "Order_options[reduce_only(0)={}]",
            self.get_reduce_only(),)
    }
}
