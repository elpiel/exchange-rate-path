use std::io;

use exchange_rate_path::graph::handler::GraphHandler;
use exchange_rate_path::{parse_line, ParsedLine};

fn main() {
    let _case_1 = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";
    let _case_1 = "2018-11-01T09:42:23+00:00 KRAKEN BTC USD 5000.0 3";
    let _case_1 = "2017-11-01T09:42:23+00:00 EXCI USD BTC 0.0006 800.0";

    // 0 - BTC 1 - USD 0-1: 1000 1-0: 0000.9
    let _case_2 = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";
    // 0 - BTC 2 - EUR 0-2: 5000 2-0: 3.0
    let _case_2 = "2018-11-01T09:42:23+00:00 KRAKEN BTC EUR 5000.0 3.0";
    // 3 - USD 4 - BTC 3-4: 0.0006 4-3: 800 | 0-4: 1 1-3: 1 4-0: 1 0-4: 1
    let _case_2 = "2017-11-01T09:42:23+00:00 EXCI USD BTC 0.0006 800.0";

    let _line = "EXCHANGE_RATE_REQUEST KRAKEN BTC EXCI USD";

    println!("The Exchange rate path problem solver");
    let mut handler = GraphHandler::default();

    for _ in 0..3 {
        println!("Enter a command:");

        let mut command_line_str = String::new();
        io::stdin()
            .read_line(&mut command_line_str)
            .expect("Reading the line failed");

        let parsed_line = match parse_line(&command_line_str) {
            Ok(parsed_line) => parsed_line,
            Err(command_error) => {
                println!(
                    "Error '{}' occurred parsing line '{}'",
                    command_error,
                    &command_line_str.trim()
                );

                continue;
            }
        };

        handle_command(&mut handler, parsed_line);
        for edge in handler.exchange_graph.graph.all_edges() {
            dbg!(edge);
        }
    }

    //    for edge in handler.graph.graph.all_edges() {
    //        dbg!(edge);
    //    }
}

fn handle_command(graph_handler: &mut GraphHandler, parsed_line: ParsedLine) {
    match parsed_line {
        ParsedLine::PriceUpdate(price_update) => {
            graph_handler.handle_update(price_update);
        }
        ParsedLine::ExchangeRequest(_exchange_request) => {}
    }
}
