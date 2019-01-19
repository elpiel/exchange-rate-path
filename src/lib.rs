use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub enum TryParseCommand {
    PriceUpdate,
    ExchangeRequest,
}

#[derive(Debug, PartialEq)]
pub struct PriceUpdate {
    pub timestamp: DateTime<Utc>,
    pub exchange: String,
    pub source_currency: String,
    pub destination_currency: String,
    pub forward_factor: f64,
    pub backward_factor: f64,
}

enum ArgumentType {
    Timestamp,
    Exchange,
    SourceCurrency,
    DestinationCurrency,
    ForwardFactor,
    BackwardFactor,
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

    pub fn from_input<'a>(input_slice: &[&str]) -> Result<Self, &'static str> {
        if input_slice.len() != PriceUpdate::POSITIONED_ARGUMENTS.len() {
            // TODO: check how can we use the constant and it's length with the format! macro
            // let error = format!("Expected {} arguments", PriceUpdate::POSITIONED_ARGUMENTS.len()).as_str();
            return Err("Expected 6 arguments");
        }

        // TODO: Proper Error Handling
        let timestamp = input_slice.get(0).unwrap().parse::<DateTime<Utc>>().unwrap();
        let exchange: String = input_slice.get(1).unwrap().parse().unwrap();
        let source_currency: String = input_slice.get(2).unwrap().parse().unwrap();
        let destination_currency: String = input_slice.get(3).unwrap().parse().unwrap();
        let forward_factor: f64 = input_slice.get(4).unwrap().parse().unwrap();
        let backward_factor: f64 = input_slice.get(5).unwrap().parse().unwrap();

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

#[derive(Debug, PartialEq)]
pub enum ParsedLine {
    PriceUpdate(PriceUpdate),
    ExchangeRequest(ExchangeRequest),
}

pub fn parse_line(input_str: &str) -> Result<ParsedLine, &str> {
    // TODO: Add custom error for wrong Command
    let line = input_str
        .lines()
        .next()
        .ok_or("No input was parsed, expected at least 1 line")
        .unwrap();
    let input: Vec<&str> = line.trim().split_whitespace().collect();

    // TODO: Add custom error for wrong Command
    let first_argument = input.get(0).ok_or("No input for the command").unwrap();
    let try_to_parse_command = which_try_to_parse_command(first_argument);

    match try_to_parse_command {
        TryParseCommand::PriceUpdate => {
            let price_update = PriceUpdate::from_input(&input).unwrap();
            Ok(ParsedLine::PriceUpdate(price_update))
        },
        TryParseCommand::ExchangeRequest => { Err("Exchange Request is not implemented")},
    }
}

fn which_try_to_parse_command(candidate: &str) -> TryParseCommand {
    if candidate == "EXCHANGE_RATE_REQUEST" {
        TryParseCommand::ExchangeRequest
    } else {
        TryParseCommand::PriceUpdate
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_parses_single_line() {
        let line = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";

        let price_update = PriceUpdate {
            timestamp: "2017-11-01T09:42:23+00:00".parse::<DateTime<Utc>>().unwrap(),
            exchange: "KRAKEN".to_owned(),
            source_currency: "BTC".to_owned(),
            destination_currency: "USD".to_owned(),
            forward_factor: 1000.0,
            backward_factor: 0.0009,
        };
        let parsed_line = ParsedLine::PriceUpdate(price_update);

        assert_eq!(Ok(parsed_line), parse_line(line));
    }

    #[test]
    fn which_try_to_command_handles_both_commands() {
        // only the &str literal "EXCHANGE_RATE_REQUEST" should end up in ExchangeRequest Enum
        assert_eq!(
            TryParseCommand::ExchangeRequest,
            which_try_to_parse_command("EXCHANGE_RATE_REQUEST")
        );
        assert_eq!(
            TryParseCommand::PriceUpdate,
            which_try_to_parse_command("does not matter")
        );
    }

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
            PriceUpdate::from_input(&vec![timestamp_str, "KRAKEN", "BTC", "USD", "1000.0", "0.0009"])
        );

        assert_eq!(Err("Expected 6 arguments"), PriceUpdate::from_input(&[]));
        assert_eq!(Err("Expected 6 arguments"), PriceUpdate::from_input(&["1", "2", "3", "4", "5"]));
        assert_eq!(Err("Expected 6 arguments"), PriceUpdate::from_input(&["1", "2", "3", "4", "5", "6", "7"]));
    }
}
