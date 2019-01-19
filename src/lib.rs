use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub enum TryParseCommand {
    PriceUpdate,
    ExchangeRequest,
}

pub struct PriceUpdate {
    timestamp: DateTime<Utc>,
    exchange: String,
    source_currency: String,
    destination_currency: String,
    forward_factor: f64,
    backward_factor: f64,
}

pub struct ExchangeRequest {
    source_exchange: String,
    source_currency: String,
    destination_exchange: String,
    destination_currency: String,
}

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
    let _try_to_parse_command =
        which_try_to_parse_command(input.get(0).ok_or("No input for the command").unwrap());

    Err("oups...")
}

fn which_try_to_parse_command(candidate: &str) -> TryParseCommand {
    if candidate == "EXCHANGE_RATE_REQUEST" {
        TryParseCommand::PriceUpdate
    } else {
        TryParseCommand::ExchangeRequest
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn which_try_to_command_handles_both_commands() {
        // only the &str literal "EXCHANGE_RATE_REQUEST" should end up in
        assert_eq!(
            TryParseCommand::PriceUpdate,
            which_try_to_parse_command("EXCHANGE_RATE_REQUEST")
        );
        assert_eq!(
            TryParseCommand::ExchangeRequest,
            which_try_to_parse_command("does not matter")
        );
    }
}
