use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 43;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Basic_order_detailEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for Basic_order_detailEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Basic_order_detailEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&mut self, value: OrderType) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn instrument_encoder(self) -> InstrumentEncoder<Self> {
            let offset = self.offset + 1;
            InstrumentEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn price_encoder(self) -> PriceEncoder<Self> {
            let offset = self.offset + 19;
            PriceEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn amount_encoder(self) -> AmountEncoder<Self> {
            let offset = self.offset + 28;
            AmountEncoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: Time_in_force) {
            let offset = self.offset + 37;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'expiry_time'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 38
        /// - encodedLength: 4
        #[inline]
        pub fn expiry_time(&mut self, value: u32) {
            let offset = self.offset + 38;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        #[inline]
        pub fn order_options(&mut self, value: Order_options) {
            let offset = self.offset + 42;
            self.get_buf_mut().put_u8_at(offset, value.0)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Basic_order_detailDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for Basic_order_detailDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Basic_order_detailDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&self) -> OrderType {
            self.get_buf().get_u8_at(self.offset).into()
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn instrument_decoder(self) -> InstrumentDecoder<Self> {
            let offset = self.offset + 1;
            InstrumentDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn price_decoder(self) -> PriceDecoder<Self> {
            let offset = self.offset + 19;
            PriceDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn amount_decoder(self) -> AmountDecoder<Self> {
            let offset = self.offset + 28;
            AmountDecoder::default().wrap(self, offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> Time_in_force {
            self.get_buf().get_u8_at(self.offset + 37).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '4294967295' }
        #[inline]
        pub fn expiry_time(&self) -> Option<u32> {
            let value = self.get_buf().get_u32_at(self.offset + 38);
            if value == 0xffffffff_u32 {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn order_options(&self) -> Order_options {
            Order_options::new(self.get_buf().get_u8_at(self.offset + 42))
        }

    }
} // end decoder mod 
