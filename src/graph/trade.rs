use std::collections::HashMap;

use petgraph::graphmap::AllEdges;
use petgraph::graphmap::DiGraphMap;
use petgraph::Directed;

use crate::graph::handler::GraphHandler;

struct Exchanger {}

type RatesHashMap = HashMap<usize, HashMap<usize, f64>>;
type NextHashMap = HashMap<usize, HashMap<usize, Option<usize>>>;

impl Exchanger {
    fn initialize_rates(edges: AllEdges<usize, f64, Directed>) -> RatesHashMap {
        edges.fold(HashMap::new(), |mut acc, (from_node, to_node, _weight)| {
            let mut dest_hash_map = HashMap::new();
            dest_hash_map.insert(to_node, 0.0);

            acc.insert(from_node, dest_hash_map);
            acc
        })
    }

    fn initialize_next(edges: AllEdges<usize, f64, Directed>) -> NextHashMap {
        edges.fold(HashMap::new(), |mut acc, (from_node, to_node, _weight)| {
            let mut dest_hash_map = HashMap::new();
            dest_hash_map.insert(to_node, None);

            acc.insert(from_node, dest_hash_map);
            acc
        })
    }

    fn new_init_rates_next(
        nodes: &[usize],
        graph: &DiGraphMap<usize, f64>,
    ) -> (RatesHashMap, NextHashMap) {
        let (mut rates, mut next) = (0..nodes.len()).fold(
            (RatesHashMap::new(), NextHashMap::new()),
            |(mut rates, mut nexts), node| {
                let (rate_init, next_init) = (0..nodes.len()).fold(
                    (HashMap::new(), HashMap::new()),
                    |(mut rate_acc, mut next_acc), dest_node| {
                        rate_acc.insert(dest_node, 0.0);
                        next_acc.insert(dest_node, None);

                        (rate_acc, next_acc)
                    },
                );
                rates.insert(node, rate_init);
                nexts.insert(node, next_init);
                (rates, nexts)
            },
        );

        for (node_1, node_2, &weight) in graph.all_edges() {
            let rate_entry = rates.entry(node_1).or_insert(HashMap::new());
            rate_entry.insert(node_2, weight);

            let next_entry = next.entry(node_1).or_insert(HashMap::new());
            next_entry.insert(node_2, Some(node_2));
        }

        (rates, next)
    }

    pub fn best_rates(graph_handler: &GraphHandler) -> NextHashMap {
        let v = &graph_handler.exchange_graph.index_map;

        let nodes = v.iter().fold(Vec::new(), |mut acc, (_key, &v)| {
            acc.push(v);
            acc
        });
        let (mut rate, mut next) =
            Self::new_init_rates_next(&nodes, &graph_handler.exchange_graph.graph);

        for k in 0..v.len() {
            for i in 0..v.len() {
                for j in 0..v.len() {
                    let mul_rate = rate.get(&i).unwrap().get(&k).unwrap()
                        * rate.get(&k).unwrap().get(&j).unwrap();

                    if rate.get(&i).unwrap().get(&j).unwrap() < &mul_rate {
                        rate.get_mut(&i).unwrap().insert(j, mul_rate);

                        let mut i_mut = next.get_mut(&i).unwrap();

                        i_mut.insert(j, *i_mut.get(&k).unwrap());
                    }
                }
            }
        }

        dbg!(&next);
        dbg!(&rate);
        next
    }

    pub fn path(from_node: usize, to_node: usize, next: &NextHashMap) -> Option<Vec<usize>> {
        if next
            .get(&from_node)
            .unwrap()
            .get(&to_node)
            .unwrap()
            .is_none()
        {
            return None;
        }

        let mut path = vec![from_node];

        let mut source_node = from_node;
        let dest_node = to_node;
        while source_node != dest_node {
            source_node = next
                .get(&source_node)
                .unwrap()
                .get(&dest_node)
                .unwrap()
                .unwrap();
            path.push(source_node);
        }

        Some(path)
    }
}

#[cfg(test)]
mod test {
    use crate::parse::command::PriceUpdate;

    use super::*;

    #[test]
    fn test_single_update_from_one_exchange() {
        let price_update = PriceUpdate::from_input(&[
            "2017-11-01T09:42:23+00:00",
            "KRAKEN",
            "USD",
            "LIT",
            "2",
            "0.5",
        ])
        .unwrap();

        let mut graph_handler: GraphHandler = GraphHandler::default();

        graph_handler.handle_update(price_update);

        let best_rates = Exchanger::best_rates(&graph_handler);

        let path = Exchanger::path(0, 1, &best_rates);
        assert_eq!(Some(vec![0_usize, 1_usize]), path);
    }

    #[test]
    fn test_two_update_from_same_exchange() {
        let price_updates = vec![
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "LIT",
                "2",
                "0.5",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "LIT",
                "EUR",
                "1.5",
                "0.666666667",
            ])
            .unwrap(),
        ];

        let mut graph_handler: GraphHandler = GraphHandler::from(price_updates);

        let best_rates = Exchanger::best_rates(&graph_handler);

        //        let path = Exchanger::path(0, 2, &best_rates);
        //        assert_eq!(Some(vec![0_usize, 1_usize, 2_usize]), path);
    }

    #[test]
    fn test_two_update_from_different_exchanges() {
        let price_updates = vec![
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "LIT",
                "0.001",
                "1000",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "EXCI",
                "LIT",
                "EUR",
                "1000",
                "0.001",
            ])
            .unwrap(),
        ];

        let mut graph_handler: GraphHandler = GraphHandler::from(price_updates);

        let best_rates = Exchanger::best_rates(&graph_handler);

        let path = Exchanger::path(0, 3, &best_rates);
        assert_eq!(Some(vec![0_usize, 1_usize, 2_usize, 3_usize]), path);
    }

    #[test]
    fn test_multiple_update_from_different_exchanges_to_different_currencies() {
        let price_updates = vec![
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "LIT",
                "0.001",
                "1000",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "GDAX",
                "LIT",
                "EUR",
                "500",
                "0.002",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "EXCI",
                "USD",
                "BTC",
                "0.005",
                "5000",
            ])
            .unwrap(),
        ];

        let mut graph_handler: GraphHandler = GraphHandler::from(price_updates);

        let best_rates = Exchanger::best_rates(&graph_handler);

        //        dbg!(best_rates);
//        let path = Exchanger::path(0, 3, &best_rates);
//        assert_eq!(Some(vec![0_usize, 1_usize, 2_usize, 3_usize]), path);

        //        let path = Exchanger::path(0, 5, &best_rates);
        //        assert_eq!(Some(vec![0_usize, 4_usize, 5_usize]), path);
    }

//    #[test]
    fn test_multiple_paths_from_different_exchanges() {
        let price_updates = vec![
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "LIT",
                "0.001",
                "1000",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "EUR",
                "2",
                "0.5",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "EXCI",
                "LIT",
                "EUR",
                "1000",
                "0.001",
            ])
            .unwrap(),
        ];

        let mut graph_handler: GraphHandler = GraphHandler::from(price_updates);

        let best_rates = Exchanger::best_rates(&graph_handler);

        let path = Exchanger::path(0, 4, &best_rates);
        assert_eq!(Some(vec![0_usize, 2_usize, 4_usize]), path);
    }

    //    #[test]
    fn test_multiple_paths_long_path_is_the_best_from_different_exchanges() {
        let price_updates = vec![
            // 0 KRAKEN USD
            // 1 KRAKEN LIT
            // 2 GDAX EUR
            // 3 GDAX LIT
            // 4 EXCI EUR
            // 5 EXCI BTC
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "LIT",
                "0.001",
                "1000",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "GDAX",
                "EUR",
                "LIT",
                "0.000666667",
                "1500",
            ])
            .unwrap(),
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "EXCI",
                "EUR",
                "BTC",
                "0.0002",
                "5000",
            ])
            .unwrap(),
        ];

        let mut graph_handler: GraphHandler = GraphHandler::from(price_updates);

        // 0
        let from_node = graph_handler
            .exchange_graph
            .index_map
            .get(&("KRAKEN".to_owned(), "USD".to_owned()))
            .unwrap();
        // 5
        let to_node = graph_handler
            .exchange_graph
            .index_map
            .get(&("EXCI".to_owned(), "BTC".to_owned()))
            .unwrap();

        // KRAKEN USD -> KRAKEN LIT 2000 * 0.001 = 2
        // KRAKEN LIT -> GDAX LIT 2 * 1.0 = 2
        // GDAX LIT -> GDAX EUR 2 * 1500 = 3000
        // GDAX EUR -> EXCI EUR 3000 * 1.0 = 3000
        // EXCI EUR -> EXCI BTC 3000 * 0.0002 = 0.6

        let best_rates = Exchanger::best_rates(&graph_handler);

        dbg!(best_rates);
        //        let path = Exchanger::path(0, 4, &best_rates);
        //        assert_eq!(Some(vec![0_usize, 1_usize, 2_usize, 3_usize, 4_usize, 5_usize]), path);
    }

    //    #[test]
    fn let_us_test() {
        let price_updates = vec![
            // KRAKEN USD -> LIT
            // 2000 * 0.25 = 625 LIT
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "LIT",
                "0.25",
                "3.2",
            ])
            .unwrap(),
            // KRAKEN USD -> EUR
            // 2000 * 0.9 = 1800 EUR
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "EUR",
                "0.9",
                "1.11",
            ])
            .unwrap(),
            // the wanted path:
            // KRAKEN USD -> KRAKEN BTC
            // 2000 * 0.001 = 2 BTC
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "USD",
                "BTC",
                "0.001",
                "1000",
            ])
            .unwrap(),
            // KRAKEN BTC -> KRAKEN ETH
            // 2 * 15 = 30 ETH
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "KRAKEN",
                "BTC",
                "ETG",
                "15",
                "0.066666667",
            ])
            .unwrap(),
            // KRAKEN ETH -> GDAX ETH
            // 30 * 1.0 = 30 ETH

            // GDAX ETH -> GDAX EUR
            // 30 * 85 = 2550 EUR
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "GDAX",
                "ETH",
                "EUR",
                "85",
                "0.012",
            ])
            .unwrap(),
            // GDAX EUR -> GDAX LIT
            // 2550 * 0.5 = 1250 LIT
            PriceUpdate::from_input(&[
                "2017-11-01T09:42:23+00:00",
                "GDAX",
                "EUR",
                "LIT",
                "0.5",
                "2.04",
            ])
            .unwrap(),
        ];
        let graph_handler: GraphHandler = GraphHandler::from(price_updates);

        let from_node = graph_handler
            .exchange_graph
            .index_map
            .get(&("KRAKEN".to_owned(), "USD".to_owned()))
            .unwrap();
        let to_node = graph_handler
            .exchange_graph
            .index_map
            .get(&("GDAX".to_owned(), "LIT".to_owned()))
            .unwrap();

        let next = Exchanger::best_rates(&graph_handler);

        let path = Exchanger::path(*from_node, *to_node, &next);

        // KRAKEN USD - 0
        // KRAKEN LIT - 1
        // KRAKEN EUR - 2
        // KRAKEN BTC - 3
        // KRAKEN ETG - 4
        // GDAX ETH - 5
        // GDAX EUR - 6
        // GDAX LIT - 7

        assert_eq!(Some(vec![0, 2, 3, 4, 5]), path);
    }

    // (KRAKEN, BTC) -> (KRAKEN, USD) 1000 - 0.0009
    // (GDAX, USD) -> (GDAX, BTC) 1001 - 0.0008

    // (GDAX, ETH) -> (GDAX, USD) 1001 - 0.0008

    // (EXCI, ETH) -> (EXCI, EUR) 500 - 0.007
    // (EXCI, BTC) -> (EXCI, EUR) 2000 - 0.0009
}
