use crate::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 74;
pub const SBE_TEMPLATE_ID: u16 = 8;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 1;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PlaceOrderMsgEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for PlaceOrderMsgEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for PlaceOrderMsgEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> PlaceOrderMsgEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        /// primitive array field 'upstreamOrderId'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn upstream_order_id(&mut self, value: [u8; 8]) {
            let offset = self.offset;
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

        /// primitive field 'timestamp'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 8
        /// - encodedLength: 4
        #[inline]
        pub fn timestamp(&mut self, value: u32) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn acc_id_encoder(self) -> AccIdEncoder<Self> {
            let offset = self.offset + 12;
            AccIdEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn order_detail_encoder(self) -> OrderDetailEncoder<Self> {
            let offset = self.offset + 21;
            OrderDetailEncoder::default().wrap(self, offset)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PlaceOrderMsgDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for PlaceOrderMsgDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for PlaceOrderMsgDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> PlaceOrderMsgDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        #[inline]
        pub fn upstream_order_id(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset),
                buf.get_u8_at(self.offset + 1),
                buf.get_u8_at(self.offset + 2),
                buf.get_u8_at(self.offset + 3),
                buf.get_u8_at(self.offset + 4),
                buf.get_u8_at(self.offset + 5),
                buf.get_u8_at(self.offset + 6),
                buf.get_u8_at(self.offset + 7),
            ]
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn timestamp(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset + 8)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn acc_id_decoder(self) -> AccIdDecoder<Self> {
            let offset = self.offset + 12;
            AccIdDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn order_detail_decoder(self) -> OrderDetailDecoder<Self> {
            let offset = self.offset + 21;
            OrderDetailDecoder::default().wrap(self, offset)
        }

    }

} // end decoder

