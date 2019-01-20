use std::error::Error;
use std::fmt;

use crate::command::{ExchangeRequest, PriceUpdate};

#[derive(Debug, PartialEq)]
pub enum TryParseCommand {
    PriceUpdate,
    ExchangeRequest,
}

pub enum ArgumentType {
    Timestamp,
    Exchange,
    SourceCurrency,
    DestinationCurrency,
    ForwardFactor,
    BackwardFactor,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum ParseErrorKind {
    NoInput,
    RequiredArgumentsCount,
    TimestampParsing,
    FloatParsing,
    StringParsing,
    // todo: remove
    ExchangeRequestUnimplemented,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct ParseCommandError(pub ParseErrorKind);

impl Error for ParseCommandError {}

impl fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_description = match self.0 {
            ParseErrorKind::NoInput => "No input for the command",
            ParseErrorKind::RequiredArgumentsCount => "Invalid number of arguments provided",
            ParseErrorKind::TimestampParsing => "Timestamp format",
            ParseErrorKind::FloatParsing => "Invalid float",
            ParseErrorKind::StringParsing => "Parsing argument failed",
            ParseErrorKind::ExchangeRequestUnimplemented => {
                "Exchange rate Request is not implemented"
            }
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
    let line = input_str
        .lines()
        .next()
        .ok_or(ParseCommandError(ParseErrorKind::NoInput))?;
    let input: Vec<&str> = line.trim().split_whitespace().collect();

    let first_argument = input
        .get(0)
        .ok_or(ParseCommandError(ParseErrorKind::NoInput))?;
    let try_to_parse_command = which_try_to_parse_command(first_argument);

    match try_to_parse_command {
        TryParseCommand::PriceUpdate => {
            let price_update = PriceUpdate::from_input(&input)?;
            Ok(ParsedLine::PriceUpdate(price_update))
        }
        TryParseCommand::ExchangeRequest => Err(ParseCommandError(
            ParseErrorKind::ExchangeRequestUnimplemented,
        )),
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
    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn parse_line_parses_single_line() {
        let line = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";

        let price_update = PriceUpdate {
            timestamp: "2017-11-01T09:42:23+00:00"
                .parse::<DateTime<Utc>>()
                .unwrap(),
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
}
