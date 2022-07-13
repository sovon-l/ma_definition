#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleOrderFlag(pub u8);
impl SimpleOrderFlag {
    #[inline]
    pub fn new(value: u8) -> Self {
        SimpleOrderFlag(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_is_trigger(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_is_trigger(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }

    #[inline]
    pub fn get_is_stop_loss(&self) -> bool {
        0 != self.0 & (1 << 1)
    }

    #[inline]
    pub fn set_is_stop_loss(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 1)
        } else {
            self.0 & !(1 << 1)
        };
        self
    }
}
impl core::fmt::Debug for SimpleOrderFlag {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "SimpleOrderFlag[is_trigger(0)={},is_stop_loss(1)={}]",
            self.get_is_trigger(),self.get_is_stop_loss(),)
    }
}
