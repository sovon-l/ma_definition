
pub fn encode_decimal<'a, P: Default + proper_ma_api::Writer<'a>>(
    get_encoder: impl FnOnce(P) -> proper_ma_api::DecEncoder<P>,
    parent_encoder: P,
    d: &rust_decimal::Decimal,
) -> P {
    let mut encoder = get_encoder(parent_encoder);
    encoder.mantissa(d.mantissa() as i64);
    encoder.exponent(d.scale() as i8);
    encoder.parent().unwrap()
}

pub fn decode_decimal<'a, P: Default + proper_ma_api::Reader<'a>>(decoder: &mut proper_ma_api::DecDecoder<P>) -> rust_decimal::Decimal {
    rust_decimal::Decimal::new(decoder.mantissa(), decoder.exponent() as u32)
}