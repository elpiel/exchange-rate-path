use crate::parse::parsing::ArgumentType;
use crate::parse::parsing::{ParseCommandError, ParseErrorKind};
use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct PriceUpdate {
    pub timestamp: DateTime<Utc>,
    pub exchange: String,
    pub source_currency: String,
    pub destination_currency: String,
    pub forward_factor: f64,
    pub backward_factor: f64,
}

impl PriceUpdate {
    // TODO: Should we use these or not?
    const POSITIONED_ARGUMENTS: [ArgumentType; 6] = [
        ArgumentType::Timestamp,
        ArgumentType::Exchange,
        ArgumentType::SourceCurrency,
        ArgumentType::DestinationCurrency,
        ArgumentType::ForwardFactor,
        ArgumentType::BackwardFactor,
    ];

    pub fn from_input<'a>(input_slice: &[&str]) -> Result<Self, ParseCommandError> {
        if input_slice.len() != PriceUpdate::POSITIONED_ARGUMENTS.len() {
            return Err(ParseCommandError(ParseErrorKind::RequiredArgumentsCount));
        }

        let timestamp = input_slice
            .get(0)
            .unwrap()
            .parse::<DateTime<Utc>>()
            .map_err(|_| ParseCommandError(ParseErrorKind::TimestampParsing))?;

        let exchange: String = input_slice.get(1).unwrap().to_string();
        let source_currency: String = input_slice.get(2).unwrap().to_string();
        let destination_currency: String = input_slice.get(3).unwrap().to_string();
        let forward_factor: f64 = input_slice
            .get(4)
            .unwrap()
            .parse()
            .map_err(|_| ParseCommandError(ParseErrorKind::FloatParsing))?;
        let backward_factor: f64 = input_slice
            .get(5)
            .unwrap()
            .parse()
            .map_err(|_| ParseCommandError(ParseErrorKind::FloatParsing))?;

        Ok(PriceUpdate {
            timestamp,
            exchange,
            source_currency,
            destination_currency,
            forward_factor,
            backward_factor,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct ExchangeRequest {
    source_exchange: String,
    source_currency: String,
    destination_exchange: String,
    destination_currency: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn price_update_error_handling() {
        let timestamp_str = "2017-11-01T09:42:23+00:00";
        let timestamp = timestamp_str.parse::<DateTime<Utc>>().unwrap();

        assert_eq!(
            Ok(PriceUpdate {
                timestamp,
                exchange: "KRAKEN".to_owned(),
                source_currency: "BTC".to_owned(),
                destination_currency: "USD".to_owned(),
                forward_factor: 1000.0,
                backward_factor: 0.0009,
            }),
            PriceUpdate::from_input(&vec![
                timestamp_str,
                "KRAKEN",
                "BTC",
                "USD",
                "1000.0",
                "0.0009"
            ])
        );
        let error_arguments_count = Err(ParseCommandError(ParseErrorKind::RequiredArgumentsCount));

        assert_eq!(error_arguments_count, PriceUpdate::from_input(&[]));
        assert_eq!(
            error_arguments_count,
            PriceUpdate::from_input(&["1", "2", "3", "4", "5"])
        );
        assert_eq!(
            error_arguments_count,
            PriceUpdate::from_input(&["1", "2", "3", "4", "5", "6", "7"])
        );

        assert_eq!(
            Err(ParseCommandError(ParseErrorKind::TimestampParsing)),
            PriceUpdate::from_input(&["1", "Exchange", "ETH", "EUR", "5.0", "6.0"])
        );

        let float_error = Err(ParseCommandError(ParseErrorKind::FloatParsing));
        assert_eq!(
            float_error,
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "Exchange",
                "ETH",
                "EUR",
                "not a float",
                "6.0"
            ])
        );
        assert_eq!(
            float_error,
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "Exchange",
                "ETH",
                "EUR",
                "5.0",
                "not a float"
            ])
        );
    }
}
