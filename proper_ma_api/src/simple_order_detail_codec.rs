use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 53;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Simple_order_detailEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for Simple_order_detailEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Simple_order_detailEncoder<P> where P: Writer<'a> + Default {
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
        pub fn trigger_encoder(self) -> TriggerEncoder<Self> {
            let offset = self.offset + 1;
            TriggerEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn triggered_ordertype_encoder(self) -> Triggered_ordertypeEncoder<Self> {
            let offset = self.offset + 10;
            Triggered_ordertypeEncoder::default().wrap(self, offset)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Simple_order_detailDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for Simple_order_detailDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Simple_order_detailDecoder<P> where P: Reader<'a> + Default {
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
        pub fn trigger_decoder(self) -> TriggerDecoder<Self> {
            let offset = self.offset + 1;
            TriggerDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn triggered_ordertype_decoder(self) -> Triggered_ordertypeDecoder<Self> {
            let offset = self.offset + 10;
            Triggered_ordertypeDecoder::default().wrap(self, offset)
        }

    }
} // end decoder mod 
