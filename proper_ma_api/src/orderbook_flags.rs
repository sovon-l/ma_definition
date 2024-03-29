#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderbookFlags(pub u8);
impl OrderbookFlags {
    #[inline]
    pub fn new(value: u8) -> Self {
        OrderbookFlags(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_is_snapshot(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_is_snapshot(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }

    #[inline]
    pub fn get_l1(&self) -> bool {
        0 != self.0 & (1 << 1)
    }

    #[inline]
    pub fn set_l1(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 1)
        } else {
            self.0 & !(1 << 1)
        };
        self
    }
}
impl core::fmt::Debug for OrderbookFlags {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "OrderbookFlags[is_snapshot(0)={},l1(1)={}]",
            self.get_is_snapshot(),self.get_l1(),)
    }
}
