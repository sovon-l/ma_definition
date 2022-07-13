
use crate::structs::market::instrument::*;

pub fn encode_instrument<'a, T:proper_ma_api::Writer<'a> + Default>(
    get_encoder: impl FnOnce(T) -> proper_ma_api::InstrumentEncoder<T>,
    parent_encoder: T,
    s: &Instrument,
) -> T {
    let mut encoder = get_encoder(parent_encoder);

    encoder.exchange(s.exchange.into());
    encoder.quote(s.quote);
    encoder.base(s.base);
    match s.instrument_type {
        InstrumentType::Spot => {
            encoder.instrument_type(proper_ma_api::instrument_type::InstrumentType::spot)
        }
        InstrumentType::Future(expiry) => {
            encoder.instrument_type(proper_ma_api::instrument_type::InstrumentType::future);
            if let Some(expiry) = expiry.as_ref() {
                encoder.expiry(*expiry);
            }
        }
    }

    encoder.parent().unwrap()
}

pub fn decode_instrument<'a, T: proper_ma_api::Reader<'a> + Default>(
    get_decoder: impl FnOnce(T) -> proper_ma_api::InstrumentDecoder<T>,
    parent_decoder: T,
) -> (Instrument, T) {
    let mut decoder = get_decoder(parent_decoder);

    (
        Instrument {
            exchange: decoder.exchange().into(),
            quote: decoder.quote(),
            base: decoder.base(),
            instrument_type: match decoder.instrument_type() {
                proper_ma_api::instrument_type::InstrumentType::spot => InstrumentType::Spot,
                proper_ma_api::instrument_type::InstrumentType::future => {
                    InstrumentType::Future(decoder.expiry())
                }
                _ => panic!(),
            },
        },
        decoder.parent().unwrap(),
    )
}
