use crate::parse::parsing::{ParseCommandError, ParseErrorKind};
use chrono::{DateTime, Utc};
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug)]
pub struct PriceUpdate {
    pub timestamp: DateTime<Utc>,
    pub exchange: String,
    pub source_currency: String,
    pub destination_currency: String,
    pub forward_factor: f64,
    pub backward_factor: f64,
}

impl Hash for PriceUpdate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timestamp.hash(state);
        self.exchange.hash(state);
        self.source_currency.hash(state);
        self.destination_currency.hash(state);
    }
}

impl PartialEq for PriceUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp)
        && self.exchange.eq(&other.exchange)
        && self.source_currency.eq(&other.source_currency)
        && self.destination_currency.eq(&other.destination_currency)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for PriceUpdate{}

impl PriceUpdate {
    pub fn new(timestamp: DateTime<Utc>, exchange: &str, source_currency: &str, destination_currency: &str, forward_factor: f64, backward_factor: f64) -> Self {
        Self {
            timestamp,
            exchange: exchange.to_owned(),
            source_currency: source_currency.to_owned(),
            destination_currency: destination_currency.to_owned(),
            forward_factor,
            backward_factor,
        }
    }

    pub fn from_input(input_slice: &[&str]) -> Result<Self, ParseCommandError> {
        if input_slice.len() != 6 {
            return Err(ParseCommandError(ParseErrorKind::RequiredArgumentsCount));
        }

        let parse_timestamp = |input: &str| -> Result<DateTime<Utc>, ParseCommandError> {
            input
                .parse::<DateTime<Utc>>()
                .map_err(|_| ParseCommandError(ParseErrorKind::TimestampParsing))
        };

        let parse_float = |input: &str| -> Result<f64, ParseCommandError> {
            input
                .parse()
                .map_err(|_| ParseCommandError(ParseErrorKind::FloatParsing))
        };

        let timestamp = parse_timestamp(input_slice[0])?;

        let exchange: String = input_slice[1].to_string();
        let source_currency: String = input_slice[2].to_string();
        let destination_currency: String = input_slice[3].to_string();
        let forward_factor: f64 = parse_float(input_slice[4])?;
        let backward_factor: f64 = parse_float(input_slice[5])?;

        Ok(Self {
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
    pub source_exchange: String,
    pub source_currency: String,
    pub destination_exchange: String,
    pub destination_currency: String,
}

impl ExchangeRequest {
    pub const COMMAND_PREFIX: &'static str = "EXCHANGE_RATE_REQUEST";

    pub fn from_input(input_slice: &[&str]) -> Result<Self, ParseCommandError> {
        if input_slice.len() != 5 {
            return Err(ParseCommandError(ParseErrorKind::RequiredArgumentsCount));
        }

        let source_exchange: String = input_slice[1].to_string();
        let source_currency: String = input_slice[2].to_string();

        let destination_exchange: String = input_slice[3].to_string();
        let destination_currency: String = input_slice[4].to_string();

        Ok(Self {
            source_exchange,
            source_currency,
            destination_exchange,
            destination_currency,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod price_update {
        use super::*;

        #[test]
        fn price_update_equality() {
            let timestamp = Utc::now();
            let comparison_update = PriceUpdate::new(timestamp.clone(), "EX1", "C1", "C2", 1.0, 2.0);
            // it should be affected by the forward and backward factors
            let equal_update = PriceUpdate::new(timestamp, "EX1", "C1", "C2", 5.0, 6.0);

            assert_eq!(equal_update, comparison_update);

            // different time
            assert_ne!(
                PriceUpdate::new(Utc::now(), "EX1", "C1", "C2", 5.0, 6.0),
                comparison_update
            );

            // different exchange
            assert_ne!(
                PriceUpdate::new(Utc::now(), "DIFF", "C1", "C2", 5.0, 6.0),
                comparison_update
            );

            // different source currency
            assert_ne!(
                PriceUpdate::new(Utc::now(), "EX1", "SOURCE", "C2", 5.0, 6.0),
                comparison_update
            );

            // different destination currency
            assert_ne!(
                PriceUpdate::new(Utc::now(), "EX1", "C1", "DEST", 5.0, 6.0),
                comparison_update
            );
        }

        #[test]
        fn price_update_valid_input() {
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
        }

        #[test]
        fn price_update_wrong_arguments_count() {
            let error_arguments_count =
                Err(ParseCommandError(ParseErrorKind::RequiredArgumentsCount));

            assert_eq!(error_arguments_count, PriceUpdate::from_input(&[]));
            assert_eq!(
                error_arguments_count,
                PriceUpdate::from_input(&["1", "2", "3", "4", "5"])
            );
            assert_eq!(
                error_arguments_count,
                PriceUpdate::from_input(&["1", "2", "3", "4", "5", "6", "7"])
            );
        }

        #[test]
        fn price_update_wrong_timestamp() {
            assert_eq!(
                Err(ParseCommandError(ParseErrorKind::TimestampParsing)),
                PriceUpdate::from_input(&["1", "Exchange", "ETH", "EUR", "5.0", "6.0"])
            );
        }

        #[test]
        fn price_update_wrong_floats() {
            let float_error = Err(ParseCommandError(ParseErrorKind::FloatParsing));
            let forward_factor_wrong = [
                "2017-11-01T09:42:23+00:00",
                "Exchange",
                "ETH",
                "EUR",
                "not a float",
                "6.0",
            ];
            assert_eq!(float_error, PriceUpdate::from_input(&forward_factor_wrong));

            let backward_factor_wrong = [
                "2017-11-01T09:42:23+00:00",
                "Exchange",
                "ETH",
                "EUR",
                "5.0",
                "not a float",
            ];

            assert_eq!(float_error, PriceUpdate::from_input(&backward_factor_wrong));
        }
    }

    mod exchange_request {
        use super::*;

        #[test]
        fn exchange_request_valid_input() {
            assert_eq!(
                Ok(ExchangeRequest {
                    source_exchange: "LACHO".to_owned(),
                    source_currency: "BTC".to_owned(),
                    destination_exchange: "KRAKEN".to_owned(),
                    destination_currency: "USD".to_owned(),
                }),
                ExchangeRequest::from_input(&vec![ExchangeRequest::COMMAND_PREFIX, "LACHO", "BTC", "KRAKEN", "USD", ])
            );
        }
    }
}
