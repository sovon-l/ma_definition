
pub fn encode_decimal<'a, P: Default + proper_ma_api::Writer<'a>>(encoder: &mut proper_ma_api::DecEncoder<P>, d: rust_decimal::Decimal) {
    encoder.mantissa(d.mantissa() as i64);
    encoder.exponent(d.scale() as i8);
}

pub fn decode_decimal<'a, P: Default + proper_ma_api::Reader<'a>>(decoder: &mut proper_ma_api::DecDecoder<P>) -> rust_decimal::Decimal {
    rust_decimal::Decimal::new(decoder.mantissa(), decoder.exponent() as u32)
}