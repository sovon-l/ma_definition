use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 9;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct AccIdEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for AccIdEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> AccIdEncoder<P> where P: Writer<'a> + Default {
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
        pub fn exchange(&mut self, value: Exchange) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive array field 'context'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: ISO_8859_1
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn context(&mut self, value: [u8; 8]) {
            let offset = self.offset + 1;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
            buf.put_u8_at(offset + 6, value[6]);
            buf.put_u8_at(offset + 7, value[7]);
        }

    }
} // end encoder mod 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct AccIdDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for AccIdDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> AccIdDecoder<P> where P: Reader<'a> + Default {
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
        pub fn exchange(&self) -> Exchange {
            self.get_buf().get_u8_at(self.offset).into()
        }

        #[inline]
        pub fn context(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 1),
                buf.get_u8_at(self.offset + 1 + 1),
                buf.get_u8_at(self.offset + 1 + 2),
                buf.get_u8_at(self.offset + 1 + 3),
                buf.get_u8_at(self.offset + 1 + 4),
                buf.get_u8_at(self.offset + 1 + 5),
                buf.get_u8_at(self.offset + 1 + 6),
                buf.get_u8_at(self.offset + 1 + 7),
            ]
        }

    }
} // end decoder mod 
