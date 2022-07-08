
// use crate::structs::gateway::order_detail::*;

// pub fn encode_order_detail_msg(buffer: &mut [u8], o: OrderDetail) {
//     let mut order_detail_msg = proper_ma_api::OrderDetailMsgEncoder::default();
//     order_detail_msg = order_detail_msg.wrap(
//         proper_ma_api::WriteBuf::new(buffer),
//         proper_ma_api::message_header_codec::ENCODED_LENGTH,
//     );
//     order_detail_msg = order_detail_msg.header(0).parent().unwrap();
    
//     let mut order_detail_encoder = order_detail_msg.order_detail_encoder();
//     encode_order_detail(order_detail_encoder, o);
// }

// pub fn encode_order_detail(order_detail_encoder: &mut proper_ma_api::OrderDetailEncoder, o: OrderDetail) {
//     match o {
//         OrderDetail::SimpleOrder(s) => {
//             let mut simple_order_detail_encoder = order_detail_encoder.simple_order_detail_encoder();
//             encode_simple_order_detail(simple_order_detail_encoder, s)
//         },
//         // OrderDetail::OCO(o1, o2) => {
//         //     let mut oco_order_detail_encoder = order_detail_encoder.oco_order_detail_encoder();
//         //     encode_simple_order_detail(&mut oco_order_detail_encoder, o1, o2);
//         // },
//         // OrderDetail::OTO(s1, s2) => encode_oto_order_detail(order_detail_encoder, s1, s2),
//         // OrderDetail::Trailing(s) => encode_trailing_order_detail(order_detail_encoder, s),
//         // OrderDetail::Adaptive(s) => encode_adaptive_order_detail(order_detail_encoder, s),
//     }
// }

// pub fn encode_simple_order_detail(
//     simple_order_detail_encoder: &mut proper_ma_api::SimpleOrderDetailEncoder,
//     o: SimpleOrderDetail,
// ) {
//     match o {
//         SimpleOrderDetail::BasicOrder {
//             basic_order_detail,
//         } => {
//             let mut basic_order_detail_encoder = simple_order_detail_encoder.execute_encoder();
//             encode_basic_order_detail(&mut basic_order_detail_encoder, basic_order_detail);
//         }
//         SimpleOrderDetail::TriggerOrder {
//             is_stop_loss,
//             trigger,
//             execute,
//         } => {
//             let mut basic_order_detail_encoder = simple_order_detail_encoder.execute_encoder();
//             encode_basic_order_detail(&mut basic_order_detail_encoder, execute);

//             let mut simple_order_flag_encoder = proper_ma_api::SimpleOrderFlat::default();
//             simple_order_flag_encoder.set_is_stop_loss(is_stop_loss);
//             simple_order_detail_encoder.simple_order_flag(simple_order_flag_encoder);

//             let mut trigger_encoder = simple_order_detail_encoder.trigger_encoder();
//             encode_decimal(trigger_encoder, trigger);
//             simple_order_detail_encoder = trigger_encoder.parent().unwrap();
//         }
//     }
// }
