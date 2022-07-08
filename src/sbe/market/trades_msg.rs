
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
    let mut symbol_e = trades_msg.instrument_encoder();
    crate::sbe::market::instrument::encode_instrument(symbol, &mut symbol_e);
    trades_msg = symbol_e.parent().unwrap();

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

        let mut price_e = trades_e.price_encoder();
        crate::sbe::decimal::encode_decimal(&mut price_e, price);
        trades_e = price_e.parent().unwrap();

        let mut size_e = trades_e.size_encoder();
        crate::sbe::decimal::encode_decimal(&mut size_e, size);
        trades_e = size_e.parent().unwrap();

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

    let mut symbol_d = trades_msg_d.instrument_decoder();
    let symbol = crate::sbe::market::instrument::decode_instrument(&mut symbol_d);
    trades_msg_d = symbol_d.parent().unwrap();

    let market_timestamp = trades_msg_d.market_timestamp();

    let mut trades_d = trades_msg_d.trades_decoder();
    let trades_count = trades_d.count();
    let mut trades = Vec::with_capacity(trades_count as usize);
    while let Ok(Some(_)) = trades_d.advance() {
        let mut trade_price_d = trades_d.price_decoder();
        let price = crate::sbe::decimal::decode_decimal(&mut trade_price_d);
        trades_d = trade_price_d.parent().unwrap();

        let mut trade_size_d = trades_d.size_decoder();
        let size = crate::sbe::decimal::decode_decimal(&mut trade_size_d);
        trades_d = trade_size_d.parent().unwrap();

        trades.push(Trade {
            price,
            size,
            timestamp: trades_d.timestamp(),
        });
    }

    Trades {
        symbol,
        market_timestamp,
        trades,
    }
}
