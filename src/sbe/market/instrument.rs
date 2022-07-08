
use crate::structs::market::instrument::*;

pub fn encode_instrument<'a, T: proper_ma_api::Writer<'a> + Default>(
    s: Instrument,
    instrument_e: &mut proper_ma_api::InstrumentEncoder<T>,
) {
    let Instrument {
        exchange,
        quote,
        base,
        instrument_type,
    } = s;
    instrument_e.exchange(exchange.into());
    instrument_e.quote(quote);
    instrument_e.base(base);
    match instrument_type {
        InstrumentType::Spot => {
            instrument_e.instrument_type(proper_ma_api::instrument_type::InstrumentType::spot)
        }
        InstrumentType::Future(expiry) => {
            instrument_e
                .instrument_type(proper_ma_api::instrument_type::InstrumentType::future);
            if let Some(expiry) = expiry {
                instrument_e.expiry(expiry);
            }
        }
    }
}

pub fn decode_instrument<'a, T: proper_ma_api::Reader<'a> + Default>(
    instrument_d: &mut proper_ma_api::InstrumentDecoder<T>,
) -> Instrument {
    Instrument {
        exchange: instrument_d.exchange().into(),
        quote: instrument_d.quote(),
        base: instrument_d.base(),
        instrument_type: match instrument_d.instrument_type() {
            proper_ma_api::instrument_type::InstrumentType::spot => InstrumentType::Spot,
            proper_ma_api::instrument_type::InstrumentType::future => {
                InstrumentType::Future(instrument_d.expiry())
            }
            _ => panic!(),
        },
    }
}
