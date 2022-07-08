#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Instrument {
    pub exchange: crate::structs::market::exchange::Exchange,
    pub base: [u8; 6],
    pub quote: [u8; 6],
    pub instrument_type: InstrumentType,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum InstrumentType {
    Spot,
    Future(Option<u32>),
}

// ----- string conv -----

impl std::fmt::Display for Instrument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}_{}-{}",
            self.exchange,
            std::str::from_utf8(&self.base).unwrap(),
            std::str::from_utf8(&self.quote).unwrap(),
            match self.instrument_type {
                InstrumentType::Spot => "0".to_string(),
                InstrumentType::Future(expiry) => match expiry {
                    Some(expiry) => format!("1_{}", expiry),
                    None => "1".to_string(),
                },
            }
        )
    }
}

impl std::str::FromStr for Instrument {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(':').collect();
        if splits.len() < 2 {
            return Err(());
        }
        let tokens: Vec<&str> = splits[1].split('-').collect();
        let parts: Vec<&str> = tokens[0].split('_').collect();
        if parts.len() < 2 {
            return Err(());
        }
        Ok(Instrument {
            exchange: crate::structs::market::exchange::Exchange::from_str(splits[0]).map_err(|_| ())?,
            base: crate::util::symbol::str_to_asset(parts[0]),
            quote: crate::util::symbol::str_to_asset(parts[1]),
            instrument_type: if tokens.len() < 2 {
                InstrumentType::Spot
            } else {
                let splits: Vec<&str> = tokens[1].split('_').collect();
                match splits[0] {
                    "0" => InstrumentType::Spot,
                    "1" => InstrumentType::Future(if splits.len() < 2 {
                        None
                    } else {
                        Some(if let Ok(v) = splits[1].parse() {
                            v
                        } else {
                            return Err(());
                        })
                    }),
                    _ => return Err(()),
                }
            },
        })
    }
}
