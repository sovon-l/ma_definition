use crate::*;

pub use encoder::*;
pub use decoder::*;

use DecEncoder as FilledAmountEncoder;
use DecEncoder as PaidAmountEncoder;
use DecEncoder as CommissionEncoder;
use DecDecoder as FilledAmountDecoder;
use DecDecoder as PaidAmountDecoder;
use DecDecoder as CommissionDecoder;

pub const ENCODED_LENGTH: usize = 45;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderProgressEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for OrderProgressEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> OrderProgressEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn filled_amount_encoder(self) -> FilledAmountEncoder<Self> {
            let offset = self.offset;
            FilledAmountEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn paid_amount_encoder(self) -> PaidAmountEncoder<Self> {
            let offset = self.offset + 9;
            PaidAmountEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn commission_encoder(self) -> CommissionEncoder<Self> {
            let offset = self.offset + 18;
            CommissionEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_status_encoder(self) -> OrderStatusEncoder<Self> {
            let offset = self.offset + 27;
            OrderStatusEncoder::default().wrap(self, offset)
        }

        /// primitive field 'lastUpdate'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 29
        /// - encodedLength: 8
        #[inline]
        pub fn last_update(&mut self, value: u64) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'lastExchangeUpdate'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 37
        /// - encodedLength: 8
        #[inline]
        pub fn last_exchange_update(&mut self, value: u64) {
            let offset = self.offset + 37;
            self.get_buf_mut().put_u64_at(offset, value);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct OrderProgressDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for OrderProgressDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> OrderProgressDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn filled_amount_decoder(self) -> FilledAmountDecoder<Self> {
            let offset = self.offset;
            FilledAmountDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn paid_amount_decoder(self) -> PaidAmountDecoder<Self> {
            let offset = self.offset + 9;
            PaidAmountDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn commission_decoder(self) -> CommissionDecoder<Self> {
            let offset = self.offset + 18;
            CommissionDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_status_decoder(self) -> OrderStatusDecoder<Self> {
            let offset = self.offset + 27;
            OrderStatusDecoder::default().wrap(self, offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn last_update(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 29)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn last_exchange_update(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 37)
        }

    }
} // end decoder mod 
