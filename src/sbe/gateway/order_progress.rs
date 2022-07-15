use crate::structs::gateway::order_progress::*;

pub fn marshal_order_progress_msg(o: &OrderProgress) -> Vec<u8> {
    let mut buffer = vec![
        0u8;
        proper_ma_api::message_header_codec::ENCODED_LENGTH
            + proper_ma_api::order_progress_msg_codec::SBE_BLOCK_LENGTH as usize
    ];

    let mut order_progress_msg_encoder = proper_ma_api::OrderProgressMsgEncoder::default();
    order_progress_msg_encoder = order_progress_msg_encoder.wrap(
        proper_ma_api::WriteBuf::new(&mut buffer),
        proper_ma_api::message_header_codec::ENCODED_LENGTH,
    );
    order_progress_msg_encoder = order_progress_msg_encoder.header(0).parent().unwrap();

    encode_order_progress(
        proper_ma_api::OrderProgressMsgEncoder::order_progress_encoder,
        order_progress_msg_encoder,
        &o,
    );

    buffer
}

pub fn encode_order_progress<'a, T: proper_ma_api::Writer<'a> + Default>(
    get_encoder: impl FnOnce(T) -> proper_ma_api::OrderProgressEncoder<T>,
    parent_encoder: T,
    o: &OrderProgress,
) -> T {
    let mut encoder = get_encoder(parent_encoder);

    encoder = crate::sbe::decimal::encode_decimal(
        proper_ma_api::OrderProgressEncoder::filled_amount_encoder,
        encoder,
        &o.filled_amount,
    );

    encoder = crate::sbe::decimal::encode_decimal(
        proper_ma_api::OrderProgressEncoder::paid_amount_encoder,
        encoder,
        &o.paid_amount,
    );

    use proper_ma_api::ErrorCode;
    use proper_ma_api::OrderStatusConcrete;
    let mut order_status_encoder = encoder.order_status_encoder();
    let (order_status_concrete, error_code) = match &o.order_status {
        OrderStatus::Submitting => (OrderStatusConcrete::Submitting, None),
        OrderStatus::Submitted => (OrderStatusConcrete::Submitted, None),
        OrderStatus::Cancelling => (OrderStatusConcrete::Cancelling, None),
        OrderStatus::Finished(err) => (
            OrderStatusConcrete::Finished,
            err.as_ref().map(|e| match e {
                ExecuteErrorCode::InsufficientFund => ErrorCode::InsufficientFund,
                ExecuteErrorCode::FailPostOnly => ErrorCode::FailPostOnly,
                ExecuteErrorCode::Timeout => ErrorCode::Timeout,
                ExecuteErrorCode::GatewayTooManyRequests => ErrorCode::GatewayTooManyRequests,
                ExecuteErrorCode::ExchangeTooManyRequests => ErrorCode::ExchangeTooManyRequests,
                ExecuteErrorCode::ExchangeConnectionError => ErrorCode::ExchangeConnectionError,
                ExecuteErrorCode::UnexpectedExchangeError => ErrorCode::UnexpectedExchangeError,
                ExecuteErrorCode::UnexpectedGatewayError => ErrorCode::UnexpectedGatewayError,
            }),
        ),
    };
    order_status_encoder.order_status_concrete(order_status_concrete);
    if let Some(error_code) = error_code {
        order_status_encoder.error_code(error_code);
    }
    encoder = order_status_encoder.parent().unwrap();

    encoder = crate::sbe::decimal::encode_decimal(
        proper_ma_api::OrderProgressEncoder::commission_encoder,
        encoder,
        &o.commission,
    );

    encoder.last_update(o.last_update);
    if let Some(last_exchange_update) = o.last_exchange_update.as_ref() {
        encoder.last_exchange_update(*last_exchange_update);
    }

    encoder.parent().unwrap()
}

pub fn unmarshal_order_progress_msg(
    msg: &[u8],
) -> Result<OrderProgress, Box<dyn std::error::Error>> {
    let mut order_progress_msg_decoder = proper_ma_api::OrderProgressMsgDecoder::default();
    let buf = proper_ma_api::ReadBuf::new(msg);

    let header = proper_ma_api::MessageHeaderDecoder::default().wrap(buf, 0);
    order_progress_msg_decoder = order_progress_msg_decoder.header(header);

    let (order_progress, _) = decode_order_progress(
        proper_ma_api::OrderProgressMsgDecoder::order_progress_decoder,
        order_progress_msg_decoder,
    );

    Ok(order_progress)
}

pub fn decode_order_progress<'a, T: proper_ma_api::Reader<'a> + Default>(
    get_decoder: impl FnOnce(T) -> proper_ma_api::OrderProgressDecoder<T>,
    parent_decoder: T,
) -> (OrderProgress, T) {
    let decoder = get_decoder(parent_decoder);

    let (filled_amount, decoder) = crate::sbe::decimal::decode_decimal(
        proper_ma_api::OrderProgressDecoder::filled_amount_decoder,
        decoder,
    );

    let (paid_amount, decoder) = crate::sbe::decimal::decode_decimal(
        proper_ma_api::OrderProgressDecoder::paid_amount_decoder,
        decoder,
    );

    let (commission, decoder) = crate::sbe::decimal::decode_decimal(
        proper_ma_api::OrderProgressDecoder::commission_decoder,
        decoder,
    );

    let mut order_status_decoder = decoder.order_status_decoder();

    use proper_ma_api::ErrorCode;
    use proper_ma_api::OrderStatusConcrete;
    let order_status = match order_status_decoder.order_status_concrete() {
        OrderStatusConcrete::Submitting => OrderStatus::Submitting,
        OrderStatusConcrete::Submitted => OrderStatus::Submitted,
        OrderStatusConcrete::Cancelling => OrderStatus::Cancelling,
        OrderStatusConcrete::Finished => {
            let error_code = order_status_decoder.error_code();
            OrderStatus::Finished(match error_code {
                ErrorCode::InsufficientFund => Some(ExecuteErrorCode::InsufficientFund),
                ErrorCode::FailPostOnly => Some(ExecuteErrorCode::FailPostOnly),
                ErrorCode::Timeout => Some(ExecuteErrorCode::Timeout),
                ErrorCode::GatewayTooManyRequests => Some(ExecuteErrorCode::GatewayTooManyRequests),
                ErrorCode::ExchangeTooManyRequests => {
                    Some(ExecuteErrorCode::ExchangeTooManyRequests)
                }
                ErrorCode::ExchangeConnectionError => {
                    Some(ExecuteErrorCode::ExchangeConnectionError)
                }
                ErrorCode::UnexpectedExchangeError => {
                    Some(ExecuteErrorCode::UnexpectedExchangeError)
                }
                ErrorCode::UnexpectedGatewayError => Some(ExecuteErrorCode::UnexpectedGatewayError),
                _ => None,
            })
        }
        _ => panic!(),
    };
    let mut decoder = order_status_decoder.parent().unwrap();

    let last_update = decoder.last_update();
    let last_exchange_update = {
        let rt = decoder.last_exchange_update();
        if rt == u64::MAX {
            None
        } else {
            Some(rt)
        }
    };

    (
        OrderProgress {
            filled_amount,
            paid_amount,
            commission,
            order_status,
            last_update,
            last_exchange_update,
        },
        decoder.parent().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn encode_decode() {
        let order_progresses = vec![OrderProgress {
            filled_amount: Decimal::new(1, 2),
            paid_amount: Decimal::new(2, 2),
            commission: Decimal::new(3, 2),
            order_status: OrderStatus::Submitting,
            last_update: 1,
            last_exchange_update: Some(2),
        }];

        for order_progress in order_progresses.iter() {
            let msg = marshal_order_progress_msg(&order_progress);
            let p2 = unmarshal_order_progress_msg(&msg).unwrap();
            assert_eq!(*order_progress, p2);
        }
    }
}
