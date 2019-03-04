use std::fmt::*;

struct BestRates {
    source_exchange: String,
    source_currency: String,
    destination_exchange: String,
    destination_currency: String,
    rate: f64,
    path: Vec<(String, String)>,
}

impl Display for BestRates {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut lines = Vec::new();
        let best_rates_begin = format!(
            "BEST_RATES_BEGIN {source_exchange} {source_currency} {destination_exchange} {destination_currency} {rate}",
            source_exchange = self.source_exchange,
            source_currency = self.source_currency,
            destination_exchange = self.destination_exchange,
            destination_currency = self.destination_currency,
            rate = self.rate,
        );

        lines.push(best_rates_begin);

        for (exchange, currency) in &self.path {
            let path_exchange = format!("{} {}", exchange, currency);
            lines.push(path_exchange);
        }

        let mut output = lines.iter().fold(String::new(), |mut acc, line| {
            let line_with_new_line = format!("{}\n", line);
            acc.push_str(line_with_new_line.as_str());
            acc
        });
        output.push_str("BEST_RATES_END");

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_displays_correctly_the_output() {
        let source_exchange = "EXCI".to_string();
        let source_currency = "EUR".to_string();

        let destination_exchange = "GDAX".to_string();
        let destination_currency = "USD".to_string();

        let best_rates = BestRates {
            source_exchange: source_exchange.clone(),
            source_currency: source_currency.clone(),
            destination_exchange: destination_exchange.clone(),
            destination_currency: destination_currency.clone(),
            rate: 14.4,
            path: vec![
                (source_exchange, source_currency),
                ("EXCH".to_string(), "EUR".to_string()),
                ("EXCH".to_string(), "BTC".to_string()),
                ("KRAKEN".to_string(), "BTC".to_string()),
                ("KRAKEN".to_string(), "USD".to_string()),
                (destination_exchange, destination_currency),
            ],
        };

        let expected = "BEST_RATES_BEGIN EXCI EUR GDAX USD 14.4
EXCI EUR
EXCH EUR
EXCH BTC
KRAKEN BTC
KRAKEN USD
GDAX USD
BEST_RATES_END";
        assert_eq!(expected, format!("{}", best_rates));
    }
}
