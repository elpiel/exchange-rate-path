use exchange_rate_path::parse_line;

fn main() {
    let line = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";

    let parsed_line = parse_line(&line);

    println!("{:?}", parsed_line);
}
