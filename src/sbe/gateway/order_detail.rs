
use crate::structs::gateway::order_detail::*;

pub fn marshal_order_detail_msg(o: OrderDetail) -> Vec<u8> {
    let mut buffer = vec![
        0u8;
        proper_ma_api::message_header_codec::ENCODED_LENGTH
            + proper_ma_api::order_detail_msg_codec::SBE_BLOCK_LENGTH as usize
    ];

    let mut order_detail_msg = proper_ma_api::OrderDetailMsgEncoder::default();
    order_detail_msg = order_detail_msg.wrap(
        proper_ma_api::WriteBuf::new(&mut buffer),
        proper_ma_api::message_header_codec::ENCODED_LENGTH,
    );
    order_detail_msg = order_detail_msg.header(0).parent().unwrap();

    let order_detail_encoder = order_detail_msg.order_detail_encoder();
    encode_order_detail(&o, order_detail_encoder);
    // order_detail_msg = order_detail_encoder.parent().unwrap();

    buffer
}

pub fn encode_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    o: &OrderDetail,
    order_detail_e: proper_ma_api::OrderDetailEncoder<T>,
) -> proper_ma_api::OrderDetailEncoder<T> {
    match o {
        OrderDetail::SimpleOrder(s) => {
            let mut simple_order_detail_e = order_detail_e.simple_order_detail_encoder();
            simple_order_detail_e = encode_simple_order_detail(s, simple_order_detail_e);
            simple_order_detail_e.parent().unwrap()            
        },
        // OrderDetail::OCO(o1, o2) => {
        //     let mut oco_order_detail_encoder = order_detail_encoder.oco_order_detail_encoder();
        //     encode_simple_order_detail(&mut oco_order_detail_encoder, o1, o2);
        // },
        // OrderDetail::OTO(s1, s2) => encode_oto_order_detail(order_detail_encoder, s1, s2),
        // OrderDetail::Trailing(s) => encode_trailing_order_detail(order_detail_encoder, s),
        // OrderDetail::Adaptive(s) => encode_adaptive_order_detail(order_detail_encoder, s),
    }
}

pub fn encode_simple_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    o: &SimpleOrderDetail,
    mut simple_order_detail_encoder: proper_ma_api::SimpleOrderDetailEncoder<T>,
) -> proper_ma_api::SimpleOrderDetailEncoder<T> {
    match o {
        SimpleOrderDetail::BasicOrder(basic_order_detail) => {
            let mut basic_order_detail_encoder = simple_order_detail_encoder.execute_encoder();
            basic_order_detail_encoder = encode_basic_order_detail(basic_order_detail, basic_order_detail_encoder);
            basic_order_detail_encoder.parent().unwrap()
        }
        SimpleOrderDetail::TriggerOrder { is_stop_loss, trigger, execute } => {
            // let mut trigger_encoder = simple_order_detail_encoder.trigger_encoder();
            // crate::sbe::decimal::encode_decimal(&mut trigger_encoder, *trigger);
            // simple_order_detail_encoder = trigger_encoder.parent().unwrap();
            simple_order_detail_encoder = crate::sbe::decimal::encode_decimal(
                proper_ma_api::SimpleOrderDetailEncoder::trigger_encoder,
                simple_order_detail_encoder,
                trigger,
            );

            let mut simple_order_flag_encoder = proper_ma_api::SimpleOrderFlag::default();
            simple_order_flag_encoder.set_is_stop_loss(*is_stop_loss);
            simple_order_detail_encoder.simple_order_flag(simple_order_flag_encoder);
            
            let mut execute_encoder = simple_order_detail_encoder.execute_encoder();
            execute_encoder = encode_basic_order_detail(execute, execute_encoder);
            execute_encoder.parent().unwrap()
        }
    }
}

pub fn encode_basic_order_detail<'a, T: proper_ma_api::Writer<'a> + Default>(
    o: &BasicOrderDetail,
    basic_order_detail_encoder: proper_ma_api::BasicOrderDetailEncoder<T>,
) -> proper_ma_api::BasicOrderDetailEncoder<T> {
    unimplemented!()   
}