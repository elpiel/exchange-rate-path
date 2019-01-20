use std::fmt;

use crate::command::{ExchangeRequest, PriceUpdate};

#[derive(Debug, PartialEq)]
pub enum TryParseCommand {
    PriceUpdate,
    ExchangeRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ParseErrorKind {
    NoInput,
    RequiredArgumentsCount,
    TimestampParsing,
    FloatParsing,
    StringParsing,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ParseCommandError(pub ParseErrorKind);

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_description = match self.0 {
            ParseErrorKind::NoInput => "No input for the command",
            ParseErrorKind::RequiredArgumentsCount => "Invalid number of arguments provided",
            ParseErrorKind::TimestampParsing => "Timestamp format",
            ParseErrorKind::FloatParsing => "Invalid float",
            ParseErrorKind::StringParsing => "Parsing argument failed",
        };

        error_description.fmt(f)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParsedLine {
    PriceUpdate(PriceUpdate),
    ExchangeRequest(ExchangeRequest),
}

pub fn parse_line(input_str: &str) -> Result<ParsedLine, ParseCommandError> {
    let no_input = ParseCommandError(ParseErrorKind::NoInput);

    let line = input_str.lines().next().ok_or(no_input)?;
    let input: Vec<&str> = line.trim().split_whitespace().collect();

    let first_argument = input.get(0).ok_or(no_input)?;
    let try_to_parse_command = which_try_to_parse_command(first_argument);

    match try_to_parse_command {
        TryParseCommand::PriceUpdate => {
            let price_update = PriceUpdate::from_input(&input)?;
            let parsed_line = ParsedLine::PriceUpdate(price_update);

            Ok(parsed_line)
        }
        TryParseCommand::ExchangeRequest => {
            let exchange_request = ExchangeRequest::from_input(&input)?;
            let parsed_line = ParsedLine::ExchangeRequest(exchange_request);

            Ok(parsed_line)
        }
    }
}

fn which_try_to_parse_command(candidate: &str) -> TryParseCommand {
    if candidate == ExchangeRequest::COMMAND_PREFIX {
        TryParseCommand::ExchangeRequest
    } else {
        TryParseCommand::PriceUpdate
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn parse_line_empty_input() {
        // for both an empty string and a space, should return an error with NoInput
        // the rest us handle by the specific Command Parsing part
        assert_eq!(
            Err(ParseCommandError(ParseErrorKind::NoInput)),
            parse_line("")
        );
        assert_eq!(
            Err(ParseCommandError(ParseErrorKind::NoInput)),
            parse_line(" ")
        );
    }

    #[test]
    fn parse_line_parses_single_line_only() {
        let line = "  2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009  \nanother line";

        assert_eq!(true, parse_line(line).is_ok());
    }

    #[test]
    fn parse_line_parses_price_update_line() {
        let line = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009 \nanother line";

        let price_update_expected = PriceUpdate {
            timestamp: "2017-11-01T09:42:23+00:00"
                .parse::<DateTime<Utc>>()
                .unwrap(),
            exchange: "KRAKEN".to_owned(),
            source_currency: "BTC".to_owned(),
            destination_currency: "USD".to_owned(),
            forward_factor: 1000.0,
            backward_factor: 0.0009,
        };
        let parsed_line_expected = ParsedLine::PriceUpdate(price_update_expected);

        assert_eq!(Ok(parsed_line_expected), parse_line(line));
    }

    #[test]
    fn parse_line_parses_exchange_rate_request_line() {
        let line = "EXCHANGE_RATE_REQUEST LACHO BTC KRAKEN USD";

        let exchange_request_expected = ExchangeRequest {
            source_exchange: "LACHO".to_owned(),
            source_currency: "BTC".to_owned(),
            destination_exchange: "KRAKEN".to_owned(),
            destination_currency: "USD".to_owned(),
        };
        let parsed_line_expected = ParsedLine::ExchangeRequest(exchange_request_expected);

        assert_eq!(Ok(parsed_line_expected), parse_line(line));
    }

    #[test]
    fn which_try_to_command_handles_both_commands() {
        // only the &str literal "EXCHANGE_RATE_REQUEST" should end up in ExchangeRequest Enum
        assert_eq!(
            TryParseCommand::ExchangeRequest,
            which_try_to_parse_command(ExchangeRequest::COMMAND_PREFIX)
        );
        assert_eq!(
            TryParseCommand::PriceUpdate,
            which_try_to_parse_command("does not matter")
        );
    }
}
