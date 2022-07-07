use crate::*;

pub use encoder::*;
pub use decoder::*;

use DecEncoder as TriggerEncoder;
use DecDecoder as TriggerDecoder;
use BasicOrderDetailEncoder as ExecuteEncoder;
use BasicOrderDetailDecoder as ExecuteDecoder;

pub const ENCODED_LENGTH: usize = 53;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SimpleOrderDetailEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for SimpleOrderDetailEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> SimpleOrderDetailEncoder<P> where P: Writer<'a> + Default {
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
        pub fn execute_encoder(self) -> ExecuteEncoder<Self> {
            let offset = self.offset;
            ExecuteEncoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn simple_order_flag(&mut self, value: SimpleOrderFlag) {
            let offset = self.offset + 43;
            self.get_buf_mut().put_u8_at(offset, value.0)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn trigger_encoder(self) -> TriggerEncoder<Self> {
            let offset = self.offset + 44;
            TriggerEncoder::default().wrap(self, offset)
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SimpleOrderDetailDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for SimpleOrderDetailDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> SimpleOrderDetailDecoder<P> where P: Reader<'a> + Default {
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
        pub fn execute_decoder(self) -> ExecuteDecoder<Self> {
            let offset = self.offset;
            ExecuteDecoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn simple_order_flag(&self) -> SimpleOrderFlag {
            SimpleOrderFlag::new(self.get_buf().get_u8_at(self.offset + 43))
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn trigger_decoder(self) -> TriggerDecoder<Self> {
            let offset = self.offset + 44;
            TriggerDecoder::default().wrap(self, offset)
        }

    }
} // end decoder mod 
