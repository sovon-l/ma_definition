
use crate::structs::market::quotes::*;

pub fn marshal_quotes_msg(q: Quotes) -> Vec<u8> {
    let mut buffer = vec![
        0u8;
        proper_ma_api::message_header_codec::ENCODED_LENGTH
            + proper_ma_api::quote_msg_codec::SBE_BLOCK_LENGTH as usize
            + proper_ma_api::quote_msg_codec::DepthsEncoder::<
                proper_ma_api::quote_msg_codec::QuoteMsgEncoder,
            >::block_length() as usize
                * q.depths.len()
            + 3
    ];
    let Quotes {
        symbol,
        market_timestamp,
        timestamp,
        is_snapshot,
        is_l1,
        depths,
    } = q;

    let mut quotes_msg = proper_ma_api::QuoteMsgEncoder::default();
    quotes_msg = quotes_msg.wrap(
        proper_ma_api::WriteBuf::new(&mut buffer),
        proper_ma_api::message_header_codec::ENCODED_LENGTH,
    );
    quotes_msg = quotes_msg.header(0).parent().unwrap();
    quotes_msg = crate::sbe::market::instrument::encode_instrument(
        proper_ma_api::QuoteMsgEncoder::instrument_encoder,
        quotes_msg,
        &symbol,
    );

    quotes_msg.market_timestamp(market_timestamp);

    if let Some(timestamp) = timestamp {
        quotes_msg.timestamp(timestamp);
    }

    let mut orderbook_flags_e = proper_ma_api::OrderbookFlags::new(0);
    orderbook_flags_e.set_is_snapshot(is_snapshot);
    orderbook_flags_e.set_l1(is_l1);
    quotes_msg.orderbook_flags(orderbook_flags_e);

    let mut depths_e = proper_ma_api::DepthsEncoder::default();
    depths_e = quotes_msg.depths_encoder(depths.len() as u8, depths_e);
    for Depth { price, size } in depths.into_iter() {
        depths_e.advance().unwrap();

        depths_e = crate::sbe::decimal::encode_decimal(
            proper_ma_api::DepthsEncoder::price_encoder,
            depths_e,
            &price,
        );

        depths_e = crate::sbe::decimal::encode_decimal(
            proper_ma_api::DepthsEncoder::size_encoder,
            depths_e,
            &size,
        );
    }
    buffer
}

pub fn unmarshal_quotes_msg(v: &[u8]) -> Quotes {
    let mut quotes_msg_d = proper_ma_api::QuoteMsgDecoder::default();
    let buf = proper_ma_api::ReadBuf::new(v);
    let header = proper_ma_api::MessageHeaderDecoder::default().wrap(buf, 0);
    quotes_msg_d = quotes_msg_d.header(header);

    let (symbol, quotes_msg_d) = crate::sbe::market::instrument::decode_instrument(
        proper_ma_api::QuoteMsgDecoder::instrument_decoder,
        quotes_msg_d,
    );

    let market_timestamp = quotes_msg_d.market_timestamp();

    let timestamp = quotes_msg_d.timestamp();

    let orderbook_flags_d = quotes_msg_d.orderbook_flags();
    let is_snapshot = orderbook_flags_d.get_is_snapshot();
    let is_l1 = orderbook_flags_d.get_l1();

    let mut depths_d = Some(quotes_msg_d.depths_decoder());
    let depths_count = depths_d.as_ref().unwrap().count();
    let mut depths = Vec::with_capacity(depths_count as usize);
    for _ in 0..depths_count {
        let mut depths_decoder = depths_d.take().unwrap();

        depths_decoder.advance().unwrap();

        let (price, depths_decoder) = crate::sbe::decimal::decode_decimal(
            proper_ma_api::DepthsDecoder::price_decoder,
            depths_decoder,
        );

        let (size, depths_decoder) = crate::sbe::decimal::decode_decimal(
            proper_ma_api::DepthsDecoder::size_decoder,
            depths_decoder,
        );

        depths.push(Depth { price, size });
        
        depths_d = Some(depths_decoder);
    }
    
    Quotes {
        symbol,
        market_timestamp,
        timestamp,
        is_snapshot,
        is_l1,
        depths,
    }
}