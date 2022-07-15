use crate::structs::market::trades::*;

pub fn marshal_trades_msg(ts: Trades) -> Vec<u8> {
    let mut buffer = vec![
        0u8;
        proper_ma_api::message_header_codec::ENCODED_LENGTH
            + proper_ma_api::trade_msg_codec::SBE_BLOCK_LENGTH as usize
            + proper_ma_api::trade_msg_codec::TradesEncoder::<
                proper_ma_api::trade_msg_codec::TradeMsgEncoder,
            >::block_length() as usize
                * ts.trades.len()
            + 3
    ];
    let Trades {
        symbol,
        market_timestamp,
        trades,
    } = ts;

    let mut trades_msg = proper_ma_api::TradeMsgEncoder::default();
    trades_msg = trades_msg.wrap(
        proper_ma_api::WriteBuf::new(&mut buffer),
        proper_ma_api::message_header_codec::ENCODED_LENGTH,
    );
    trades_msg = trades_msg.header(0).parent().unwrap();
    trades_msg = crate::sbe::market::instrument::encode_instrument(
        proper_ma_api::TradeMsgEncoder::instrument_encoder,
        trades_msg,
        &symbol,
    );

    trades_msg.market_timestamp(market_timestamp);

    let mut trades_e = proper_ma_api::TradesEncoder::default();
    trades_e = trades_msg.trades_encoder(trades.len() as u8, trades_e);
    for Trade {
        price,
        size,
        timestamp,
    } in trades.into_iter()
    {
        trades_e.advance().unwrap();

        trades_e = crate::sbe::decimal::encode_decimal(
            proper_ma_api::TradesEncoder::price_encoder,
            trades_e,
            &price,
        );

        trades_e = crate::sbe::decimal::encode_decimal(
            proper_ma_api::TradesEncoder::size_encoder,
            trades_e,
            &size,
        );

        trades_e.timestamp(timestamp);
    }
    // trades_msg = trades_e.parent().unwrap();
    buffer
}

pub fn unmarshal_trades_msg(v: &[u8]) -> Trades {
    let mut trades_msg_d = proper_ma_api::TradeMsgDecoder::default();
    let buf = proper_ma_api::ReadBuf::new(v);
    let header = proper_ma_api::MessageHeaderDecoder::default().wrap(buf, 0);
    trades_msg_d = trades_msg_d.header(header);

    let (symbol, trades_msg_d) = crate::sbe::market::instrument::decode_instrument(
        proper_ma_api::TradeMsgDecoder::instrument_decoder,
        trades_msg_d,
    );

    let market_timestamp = trades_msg_d.market_timestamp();

    let mut trades_d = Some(trades_msg_d.trades_decoder());
    let trades_count = trades_d.as_ref().unwrap().count();
    let mut trades = Vec::with_capacity(trades_count as usize);
    // while let Ok(Some(_)) = trades_d.advance() {
    for _ in 0..trades_count {
        let mut trades_decoder = trades_d.take().unwrap();

        trades_decoder.advance().unwrap();

        let (price, trades_decoder) = crate::sbe::decimal::decode_decimal(
            proper_ma_api::TradesDecoder::price_decoder,
            trades_decoder,
        );

        let (size, trades_decoder) = crate::sbe::decimal::decode_decimal(
            proper_ma_api::TradesDecoder::size_decoder,
            trades_decoder,
        );

        trades.push(Trade {
            price,
            size,
            timestamp: trades_decoder.timestamp(),
        });

        trades_d = Some(trades_decoder);
    }

    Trades {
        symbol,
        market_timestamp,
        trades,
    }
}
