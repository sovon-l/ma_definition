use crate::structs::gateway::order_detail::*;

pub fn marshal_order_detail_msg(o: &OrderDetail) -> Vec<u8> {
    let mut buffer = vec![
        0u8;
        proper_ma_api::message_header_codec::ENCODED_LENGTH
            + proper_ma_api::order_detail_msg_codec::SBE_BLOCK_LENGTH as usize
    ];

    let mut order_detail_msg_encoder = proper_ma_api::OrderDetailMsgEncoder::default();
    order_detail_msg_encoder = order_detail_msg_encoder.wrap(
        proper_ma_api::WriteBuf::new(&mut buffer),
        proper_ma_api::message_header_codec::ENCODED_LENGTH,
    );
    order_detail_msg_encoder = order_detail_msg_encoder.header(0).parent().unwrap();

    encode_order_detail(
        proper_ma_api::OrderDetailMsgEncoder::order_detail_encoder,
        order_detail_msg_encoder,
        &o,
    );

    buffer
}

pub fn encode_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    get_encoder: impl FnOnce(T) -> proper_ma_api::OrderDetailEncoder<T>,
    parent_encoder: T,
    o: &OrderDetail,
) -> T {
    let encoder = get_encoder(parent_encoder);
    match o {
        OrderDetail::SimpleOrder(s) => encode_simple_order_detail(
            proper_ma_api::OrderDetailEncoder::simple_order_detail_encoder,
            encoder,
            s,
        ),
        // OrderDetail::OCO(o1, o2) => {
        //     let mut oco_order_detail_encoder = order_detail_encoder.oco_order_detail_encoder();
        //     encode_simple_order_detail(&mut oco_order_detail_encoder, o1, o2);
        // },
        // OrderDetail::OTO(s1, s2) => encode_oto_order_detail(order_detail_encoder, s1, s2),
        // OrderDetail::Trailing(s) => encode_trailing_order_detail(order_detail_encoder, s),
        // OrderDetail::Adaptive(s) => encode_adaptive_order_detail(order_detail_encoder, s),
    }
    .parent()
    .unwrap()
}

pub fn encode_simple_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    get_encoder: impl FnOnce(T) -> proper_ma_api::SimpleOrderDetailEncoder<T>,
    parent_encoder: T,
    o: &SimpleOrderDetail,
) -> T {
    let mut encoder = get_encoder(parent_encoder);
    match o {
        SimpleOrderDetail::BasicOrder(basic_order_detail) => encode_basic_order_detail(
            proper_ma_api::SimpleOrderDetailEncoder::execute_encoder,
            encoder,
            basic_order_detail,
        ),
        SimpleOrderDetail::TriggerOrder {
            is_stop_loss,
            trigger,
            execute,
        } => {
            encoder = crate::sbe::decimal::encode_decimal(
                proper_ma_api::SimpleOrderDetailEncoder::trigger_encoder,
                encoder,
                trigger,
            );

            let mut simple_order_flag_encoder = proper_ma_api::SimpleOrderFlag::default();
            simple_order_flag_encoder.set_is_trigger(true);
            simple_order_flag_encoder.set_is_stop_loss(*is_stop_loss);
            encoder.simple_order_flag(simple_order_flag_encoder);

            encode_basic_order_detail(
                proper_ma_api::SimpleOrderDetailEncoder::execute_encoder,
                encoder,
                execute,
            )
        }
    }
    .parent()
    .unwrap()
}

pub fn encode_basic_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    get_encoder: impl FnOnce(T) -> proper_ma_api::BasicOrderDetailEncoder<T>,
    parent_encoder: T,
    o: &BasicOrderDetail,
) -> T {
    let mut encoder = get_encoder(parent_encoder);

    match o {
        BasicOrderDetail::Limit {
            instrument,
            price,
            amount,
            time_in_force,
            post_only,
            reduce_only,
        } => {
            encoder.order_type(proper_ma_api::OrderType::limit);

            encoder = crate::sbe::market::instrument::encode_instrument(
                proper_ma_api::BasicOrderDetailEncoder::instrument_encoder,
                encoder,
                instrument,
            );

            encoder = crate::sbe::decimal::encode_decimal(
                proper_ma_api::BasicOrderDetailEncoder::price_encoder,
                encoder,
                price,
            );

            encoder = crate::sbe::decimal::encode_decimal(
                proper_ma_api::BasicOrderDetailEncoder::amount_encoder,
                encoder,
                amount,
            );

            match time_in_force {
                TimeInForce::GoodTill(time) => {
                    if let Some(time) = time.as_ref() {
                        encoder.expiry_time(*time);
                    }
                    encoder.time_in_force(proper_ma_api::TimeInForce::GoodTill);
                }
                TimeInForce::ImmediateOrCancel => {
                    encoder.time_in_force(proper_ma_api::TimeInForce::IOC);
                }
                TimeInForce::FillOrKill => {
                    encoder.time_in_force(proper_ma_api::TimeInForce::FOK);
                }
            }

            let mut order_options_encoder = proper_ma_api::OrderOptions::default();
            order_options_encoder.set_reduce_only(*reduce_only);
            order_options_encoder.set_post_only(*post_only);
            encoder.order_options(order_options_encoder);
        }
        BasicOrderDetail::Market {
            instrument,
            amount,
            reduce_only,
        } => {
            encoder.order_type(proper_ma_api::OrderType::market);

            encoder = crate::sbe::market::instrument::encode_instrument(
                proper_ma_api::BasicOrderDetailEncoder::instrument_encoder,
                encoder,
                instrument,
            );

            encoder = crate::sbe::decimal::encode_decimal(
                proper_ma_api::BasicOrderDetailEncoder::amount_encoder,
                encoder,
                amount,
            );

            let mut order_options_encoder = proper_ma_api::OrderOptions::default();
            order_options_encoder.set_reduce_only(*reduce_only);
            encoder.order_options(order_options_encoder);
        }
    }

    encoder.parent().unwrap()
}

pub fn unmarshal_order_detail_msg(msg: &[u8]) -> Result<OrderDetail, Box<dyn std::error::Error>> {
    let mut order_detail_msg_decoder = proper_ma_api::OrderDetailMsgDecoder::default();
    let buf = proper_ma_api::ReadBuf::new(msg);

    let header = proper_ma_api::MessageHeaderDecoder::default().wrap(buf, 0);
    order_detail_msg_decoder = order_detail_msg_decoder.header(header);

    let (order_detail, _) = decode_order_detail(
        proper_ma_api::OrderDetailMsgDecoder::order_detail_decoder,
        order_detail_msg_decoder,
    );

    Ok(order_detail)
}

pub fn decode_order_detail<'a, T: proper_ma_api::Reader<'a> + Default>(
    get_decoder: impl FnOnce(T) -> proper_ma_api::OrderDetailDecoder<T>,
    parent_decoder: T,
) -> (OrderDetail, T) {
    let decoder = get_decoder(parent_decoder);
    let (rt, mut decoder) = decode_simple_order_detail(
        proper_ma_api::OrderDetailDecoder::simple_order_detail_decoder,
        decoder,
    );
    (OrderDetail::SimpleOrder(rt), decoder.parent().unwrap())
}

pub fn decode_simple_order_detail<'a, T: proper_ma_api::Reader<'a> + Default>(
    get_decoder: impl FnOnce(T) -> proper_ma_api::SimpleOrderDetailDecoder<T>,
    parent_decoder: T,
) -> (SimpleOrderDetail, T) {
    let decoder = get_decoder(parent_decoder);

    let simple_order_flag = decoder.simple_order_flag();

    if simple_order_flag.get_is_trigger() {
        let (execute, decoder) = decode_basic_order_detail(
            proper_ma_api::SimpleOrderDetailDecoder::execute_decoder,
            decoder,
        );
        let (trigger, mut decoder) = crate::sbe::decimal::decode_decimal(
            proper_ma_api::SimpleOrderDetailDecoder::trigger_decoder,
            decoder,
        );
        (
            SimpleOrderDetail::TriggerOrder {
                trigger,
                is_stop_loss: simple_order_flag.get_is_stop_loss(),
                execute,
            },
            decoder.parent().unwrap(),
        )
    } else {
        let (execute, mut decoder) = decode_basic_order_detail(
            proper_ma_api::SimpleOrderDetailDecoder::execute_decoder,
            decoder,
        );
        (
            SimpleOrderDetail::BasicOrder(execute),
            decoder.parent().unwrap(),
        )
    }
}

pub fn decode_basic_order_detail<'a, T: proper_ma_api::Reader<'a> + Default>(
    get_decoder: impl FnOnce(T) -> proper_ma_api::BasicOrderDetailDecoder<T>,
    parent_decoder: T,
) -> (BasicOrderDetail, T) {
    let decoder = get_decoder(parent_decoder);

    let (amount, decoder) = crate::sbe::decimal::decode_decimal(
        proper_ma_api::BasicOrderDetailDecoder::amount_decoder,
        decoder,
    );

    let order_options = decoder.order_options();

    let (instrument, mut decoder) = crate::sbe::market::instrument::decode_instrument(
        proper_ma_api::BasicOrderDetailDecoder::instrument_decoder,
        decoder,
    );

    match decoder.order_type() {
        proper_ma_api::OrderType::limit => {
            let (price, mut decoder) = crate::sbe::decimal::decode_decimal(
                proper_ma_api::BasicOrderDetailDecoder::price_decoder,
                decoder,
            );
            let time_in_force = decoder.time_in_force();

            (
                BasicOrderDetail::Limit {
                    instrument,
                    price,
                    amount,
                    time_in_force: match time_in_force {
                        proper_ma_api::TimeInForce::GoodTill => {
                            TimeInForce::GoodTill(decoder.expiry_time())
                        }
                        proper_ma_api::TimeInForce::IOC => TimeInForce::ImmediateOrCancel,
                        proper_ma_api::TimeInForce::FOK => TimeInForce::FillOrKill,
                        _ => TimeInForce::GoodTill(None),
                    },
                    reduce_only: order_options.get_reduce_only(),
                    post_only: order_options.get_post_only(),
                },
                decoder.parent().unwrap(),
            )
        }
        proper_ma_api::OrderType::market => (
            BasicOrderDetail::Market {
                instrument,
                amount,
                reduce_only: order_options.get_reduce_only(),
            },
            decoder.parent().unwrap(),
        ),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn encode_decode() {
        let order_details = vec![
            OrderDetail::SimpleOrder(SimpleOrderDetail::TriggerOrder {
                trigger: Decimal::new(1, 2),
                is_stop_loss: true,
                execute: BasicOrderDetail::Limit {
                    instrument: crate::structs::market::instrument::Instrument::from_str(
                        "binance:btc_usdt",
                    )
                    .unwrap(),
                    price: Decimal::new(1, 2),
                    amount: Decimal::new(1, 2),
                    time_in_force: TimeInForce::GoodTill(Some(1)),
                    reduce_only: false,
                    post_only: false,
                },
            }),
            OrderDetail::SimpleOrder(SimpleOrderDetail::BasicOrder(BasicOrderDetail::Limit {
                instrument: crate::structs::market::instrument::Instrument::from_str(
                    "binance:btc_usdt",
                )
                .unwrap(),
                price: Decimal::new(1, 2),
                amount: Decimal::new(1, 2),
                time_in_force: TimeInForce::GoodTill(Some(1)),
                reduce_only: false,
                post_only: false,
            })),
        ];

        for order_detail in order_details.iter() {
            let msg = marshal_order_detail_msg(&order_detail);
            let unmarshaled = unmarshal_order_detail_msg(&msg).unwrap();
            assert_eq!(*order_detail, unmarshaled);
        }
    }
}
