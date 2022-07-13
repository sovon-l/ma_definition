use crate::*;

pub use encoder::*;
pub use decoder::*;

use DecEncoder as PriceEncoder;
use DecEncoder as AmountEncoder;
use DecDecoder as PriceDecoder;
use DecDecoder as AmountDecoder;

pub const ENCODED_LENGTH: usize = 47;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct BasicOrderDetailEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for BasicOrderDetailEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> BasicOrderDetailEncoder<P> where P: Writer<'a> + Default {
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
        pub fn time_in_force(&mut self, value: TimeInForce) {
            let offset = self.offset + 37;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'expiryTime'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 38
        /// - encodedLength: 8
        #[inline]
        pub fn expiry_time(&mut self, value: u64) {
            let offset = self.offset + 38;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        #[inline]
        pub fn order_options(&mut self, value: OrderOptions) {
            let offset = self.offset + 46;
            self.get_buf_mut().put_u8_at(offset, value.0)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct BasicOrderDetailDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for BasicOrderDetailDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> BasicOrderDetailDecoder<P> where P: Reader<'a> + Default {
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
        pub fn time_in_force(&self) -> TimeInForce {
            self.get_buf().get_u8_at(self.offset + 37).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-1' }
        #[inline]
        pub fn expiry_time(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 38);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        #[inline]
        pub fn order_options(&self) -> OrderOptions {
            OrderOptions::new(self.get_buf().get_u8_at(self.offset + 46))
        }

    }
} // end decoder mod 
