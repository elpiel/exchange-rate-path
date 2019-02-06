use std::io;
use std::str::FromStr;

use petgraph::graphmap::DiGraphMap;
use petgraph::graphmap::GraphMap;

use exchange_rate_path::command::PriceUpdate;
use exchange_rate_path::graph::collection::IndexHashMap;
use exchange_rate_path::{parse_line, ParseCommandError, ParsedLine};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Exchange {
    name: String,
}

impl Exchange {
    pub fn new(exchange: String) -> Self {
        Self { name: exchange }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Currency {
    pub name: String,
}

impl Currency {
    pub fn new(currency: String) -> Self {
        Self { name: currency }
    }
}

//struct ExchangeGraph<'a> {
//    graph: DiGraphMap<(&'a str, &'a str), f64>
//}

type GraphType<'a> = DiGraphMap<usize, f64>;

fn main() {
    let _line = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";
    let _line = "2017-11-01T09:42:23+00:00 EXCI USD BTC 0.0006 800.0";
    let _line = "EXCHANGE_RATE_REQUEST KRAKEN BTC EXCI USD";

    println!("The Exchange rate path problem solver");
    let mut graph: GraphType = GraphMap::new();
    let mut index_map: IndexHashMap<(String, String)> = IndexHashMap::new();

    let mut read_lines: Vec<ParsedLine> = Vec::new();

    for _ in 0..2 {
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

        handle_command(&mut graph, &mut index_map, parsed_line);
    }

    println!("{:?}", index_map);
    println!("{:?}", graph);
}

fn add_price_update<'a>(
    graph: &'a mut GraphType<'a>,
    index_map: &mut IndexHashMap<(String, String)>,
    price_update: PriceUpdate,
) {
    let source_node = (
        price_update.exchange.clone(),
        price_update.source_currency.clone(),
    );
    let dest_node = (
        price_update.exchange.clone(),
        price_update.destination_currency.clone(),
    );

    let source_node_index = index_map.entry(source_node);
    let dest_node_index = index_map.entry(dest_node);

    let source_node = graph.add_node(source_node_index);
    let dest_node = graph.add_node(dest_node_index);

    let source_to_dest = graph.add_edge(source_node, dest_node, price_update.forward_factor);
    let source_to_dest = graph.add_edge(dest_node, source_node, price_update.backward_factor);
}

fn handle_command<'a>(
    graph: &mut GraphType<'a>,
    index_map: &mut IndexHashMap<(String, String)>,
    parsed_line: ParsedLine,
) {
    match parsed_line {
        ParsedLine::PriceUpdate(price_update) => {
            add_price_update(graph, index_map, price_update);
        }
        ParsedLine::ExchangeRequest(exchange_request) => {}
    }
}
