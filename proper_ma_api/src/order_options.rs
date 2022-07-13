#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderOptions(pub u8);
impl OrderOptions {
    #[inline]
    pub fn new(value: u8) -> Self {
        OrderOptions(value)
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

    #[inline]
    pub fn get_post_only(&self) -> bool {
        0 != self.0 & (1 << 1)
    }

    #[inline]
    pub fn set_post_only(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 1)
        } else {
            self.0 & !(1 << 1)
        };
        self
    }
}
impl core::fmt::Debug for OrderOptions {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "OrderOptions[reduce_only(0)={},post_only(1)={}]",
            self.get_reduce_only(),self.get_post_only(),)
    }
}
