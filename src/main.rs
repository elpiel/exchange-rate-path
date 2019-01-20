use exchange_rate_path::parse_line;
use exchange_rate_path::ParsedLine;
use exchange_rate_path::ParseCommandError;
use petgraph::Graph;
use std::io;

fn main() {
    let _line = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";

    println!("The Exchange rate path problem solver");

    loop {
        println!("Enter a command:");

        let mut command_line_str = String::new();
        io::stdin().read_line(&mut command_line_str)
            .expect("Reading the line failed");

        let parsed_line = match parse_line(&command_line_str) {
            Ok(parsed_line) => parsed_line,
            Err(command_error) => {
                println!("Error '{}' occurred parsing line '{}'", command_error, command_line_str.trim());

                continue;
            }
        };

        println!("{:?}", parsed_line);
    }
}
