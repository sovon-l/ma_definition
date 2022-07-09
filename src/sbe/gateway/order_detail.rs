
use crate::structs::gateway::order_detail::*;

pub fn marshal_order_detail_msg(o: OrderDetail) -> Vec<u8> {
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
        OrderDetail::SimpleOrder(s) => {     
            encode_simple_order_detail(
                proper_ma_api::OrderDetailEncoder::simple_order_detail_encoder,
                encoder,
                s,
            )
        },
        // OrderDetail::OCO(o1, o2) => {
        //     let mut oco_order_detail_encoder = order_detail_encoder.oco_order_detail_encoder();
        //     encode_simple_order_detail(&mut oco_order_detail_encoder, o1, o2);
        // },
        // OrderDetail::OTO(s1, s2) => encode_oto_order_detail(order_detail_encoder, s1, s2),
        // OrderDetail::Trailing(s) => encode_trailing_order_detail(order_detail_encoder, s),
        // OrderDetail::Adaptive(s) => encode_adaptive_order_detail(order_detail_encoder, s),
    }.parent().unwrap()
}

pub fn encode_simple_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    get_encoder: impl FnOnce(T) -> proper_ma_api::SimpleOrderDetailEncoder<T>,
    parent_encoder: T,
    o: &SimpleOrderDetail,
) -> T {
    let mut encoder = get_encoder(parent_encoder);
    match o {
        SimpleOrderDetail::BasicOrder(basic_order_detail) => {
            encode_basic_order_detail(
                proper_ma_api::SimpleOrderDetailEncoder::execute_encoder,
                encoder,
                basic_order_detail,
            )
        }
        SimpleOrderDetail::TriggerOrder { is_stop_loss, trigger, execute } => {
            encoder = crate::sbe::decimal::encode_decimal(
                proper_ma_api::SimpleOrderDetailEncoder::trigger_encoder,
                encoder,
                trigger,
            );

            let mut simple_order_flag_encoder = proper_ma_api::SimpleOrderFlag::default();
            simple_order_flag_encoder.set_is_stop_loss(*is_stop_loss);
            encoder.simple_order_flag(simple_order_flag_encoder);
            
            encode_basic_order_detail(
                proper_ma_api::SimpleOrderDetailEncoder::execute_encoder,
                encoder,
                execute,
            )
        }
    }.parent().unwrap()
}

pub fn encode_basic_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    get_encoder: impl FnOnce(T) -> proper_ma_api::BasicOrderDetailEncoder<T>,
    parent_encoder: T,
    o: &BasicOrderDetail,
) -> T {
    unimplemented!()   
}