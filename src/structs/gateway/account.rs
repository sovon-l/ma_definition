#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Account {
    pub exchange: crate::structs::market::exchange::Exchange,
    pub context: String, // TODO: enum? u8?
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}_{}", self.exchange, self.context)
    }
}

impl std::str::FromStr for Account {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('_');
        let exchange = parts.next().ok_or("missing exchange")?;
        let context = parts.next().ok_or("missing context")?;
        let exchange = crate::structs::market::exchange::Exchange::from_str(exchange)?;
        Ok(Account {
            exchange,
            context: context.to_string(),
        })
    }
}

// implementing deserialize on account
impl<'de> serde::Deserialize<'de> for Account {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(s.parse().map_err(serde::de::Error::custom)?)
    }
}
