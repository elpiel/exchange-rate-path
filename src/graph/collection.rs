use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::hash::Hash;

use petgraph::graphmap::DiGraphMap;

use crate::command::PriceUpdate;
use petgraph::graphmap::AllEdges;
use petgraph::Directed;

#[derive(Debug, PartialEq)]
pub enum Index {
    Fetched(usize),
    Inserted(usize),
}

impl Index {
    pub fn get_value(&self) -> &usize {
        match self {
            Index::Fetched(index) => index,
            Index::Inserted(index) => index,
        }
    }
}

#[derive(Debug)]
pub struct IndexHashMap<K>
where
    K: Eq + Hash,
{
    hash_map: HashMap<K, usize>,
    indices: Vec<K>,
}

impl<K> IndexHashMap<K>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::<K, usize>::new(),
            indices: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<K, usize> {
        self.hash_map.iter()
    }

    pub fn entry(&mut self, key: K) -> Index {
        let hash_key = key.clone();

        match self.hash_map.get(&hash_key) {
            Some(index) => Index::Fetched(*index),
            None => {
                self.indices.push(key);
                let index = self.indices.len() - 1;
                self.hash_map.insert(hash_key, index);

                Index::Inserted(index)
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&usize> {
        self.hash_map.get(key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.hash_map.contains_key(key)
    }

    pub fn get_index(&self, index: &usize) -> Option<&K> {
        self.indices.get(*index)
    }

    pub fn contains_index(&self, index: &usize) -> bool {
        self.indices.get(*index).is_some()
    }

    pub fn len(&self) -> usize {
        self.indices.len()
    }
}

#[derive(Debug)]
pub struct ExchangeCompleteGraph {
    pub graph: DiGraphMap<usize, f64>,
    pub index_map: IndexHashMap<(String, String)>,
}

impl Default for ExchangeCompleteGraph {
    fn default() -> Self {
        Self {
            graph: DiGraphMap::default(),
            index_map: IndexHashMap::new(),
        }
    }
}

impl ExchangeCompleteGraph {
    pub fn add(&mut self, price_update: &PriceUpdate) -> (Index, Index) {
        let source_node = (
            price_update.exchange.clone(),
            price_update.source_currency.clone(),
        );
        let dest_node = (
            price_update.exchange.clone(),
            price_update.destination_currency.clone(),
        );

        let source_index = self.index_map.entry(source_node);
        let dest_index = self.index_map.entry(dest_node);

        match (&source_index, &dest_index) {
            (&Index::Fetched(source_index), &Index::Fetched(dest_index)) => {
                self.graph
                    .add_edge(source_index, dest_index, price_update.forward_factor);
                self.graph
                    .add_edge(dest_index, source_index, price_update.backward_factor);
            }
            (&Index::Fetched(source_index), &Index::Inserted(dest_index)) => {
                self.insert_for_exchange(&price_update, dest_index, Some(source_index), false);
            }
            (&Index::Inserted(source_index), &Index::Fetched(dest_index)) => {
                self.insert_for_exchange(price_update, source_index, Some(dest_index), true);
            }
            (&Index::Inserted(source_index), &Index::Inserted(dest_index)) => {
                // first insert the source node without a source
                self.insert_for_exchange(&price_update, source_index, None, true);
                // then add the destination node and add the edge & weight between source <-> destination
                self.insert_for_exchange(&price_update, dest_index, Some(source_index), false);
            }
        }

        (source_index, dest_index)
    }

    /// Inserts a given exchange in the graph
    /// if `is_forward` is `true`, then the factors should be applied from node -> origin forward and origin-> node
    fn insert_for_exchange(
        &mut self,
        price_update: &PriceUpdate,
        node: usize,
        origin: Option<usize>,
        is_forward: bool,
    ) {
        let currency = if is_forward {
            &price_update.source_currency
        } else {
            &price_update.destination_currency
        };

        let same_exchange =
            self.index_map
                .iter()
                .filter(|((_, exchange_currency), &node_index)| {
                    currency == exchange_currency && node_index != node
                });

        for (_, &exchange_node) in same_exchange {
            self.graph.add_edge(node, exchange_node, 1.0);
            self.graph.add_edge(exchange_node, node, 1.0);
        }

        match origin {
            Some(origin_index) => {
                let forward_factor = if is_forward {
                    price_update.forward_factor
                } else {
                    price_update.backward_factor
                };
                let backward_factor = if is_forward {
                    price_update.backward_factor
                } else {
                    price_update.forward_factor
                };

                // Node -> origin is forward if `is_forward` is `true`
                self.graph.add_edge(node, origin_index, forward_factor);
                self.graph.add_edge(origin_index, node, backward_factor);
            }
            None => {}
        }
    }

    pub fn get_edges(&self) -> AllEdges<usize, f64, Directed> {
        self.graph.all_edges()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod exchange_complete_graph {
        use chrono::prelude::*;

        use super::*;

        #[test]
        fn second_price_update_fetches_the_nodes_and_updates_the_weights() {
            let mut exchange_graph = ExchangeCompleteGraph::default();

            let price_update = PriceUpdate::new(
                Utc.ymd(2017, 11, 1).and_hms(22, 10, 0),
                "KRAKEN",
                "BTC",
                "USD",
                1000.0,
                0.0009,
            );

            exchange_graph.add(&price_update);

            let updated_factors = PriceUpdate::new(Utc::now(), "KRAKEN", "BTC", "USD", 5000.0, 3.0);

            exchange_graph.add(&updated_factors);
            let source_key = (
                updated_factors.exchange.clone(),
                updated_factors.source_currency,
            );
            let dest_key = (
                updated_factors.exchange.clone(),
                updated_factors.destination_currency,
            );

            assert_eq!(2, exchange_graph.index_map.len());

            let source_node = exchange_graph.index_map.get(&source_key);
            let dest_node = exchange_graph.index_map.get(&dest_key);

            // first check if keys are inserted and returned correctly
            assert_eq!(Some(&0_usize), source_node);
            assert_eq!(Some(&1_usize), dest_node);

            // check values for factor
            assert_eq!(
                Some(&5000.0),
                exchange_graph
                    .graph
                    .edge_weight(*source_node.unwrap(), *dest_node.unwrap())
            );
            assert_eq!(
                Some(&3.0),
                exchange_graph
                    .graph
                    .edge_weight(*dest_node.unwrap(), *source_node.unwrap())
            );
        }

        #[test]
        fn second_price_update_inserts_node_for_different_exchange_and_links_same_currencies_with_default_weight(
        ) {
            let mut exchange_graph = ExchangeCompleteGraph::default();

            let first_update = PriceUpdate::new(Utc::now(), "KRAKEN", "BTC", "USD", 1000.0, 0.0009);

            let (node_0, node_1) = exchange_graph.add(&first_update);

            let second_update = PriceUpdate::new(Utc::now(), "EXCI", "BTC", "EUR", 5000.0, 2.0);

            let (node_2, node_3) = exchange_graph.add(&second_update);

            assert_eq!(4, exchange_graph.index_map.len());

            assert_eq!(6, exchange_graph.graph.all_edges().count());
            // first price update edges
            assert_eq!(
                Some(&1000.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_0.get_value(), *node_1.get_value())
            );
            assert_eq!(
                Some(&0.0009),
                exchange_graph
                    .graph
                    .edge_weight(*node_1.get_value(), *node_0.get_value())
            );
            // second price update edges
            assert_eq!(
                Some(&5000.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_2.get_value(), *node_3.get_value())
            );
            assert_eq!(
                Some(&2.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_3.get_value(), *node_2.get_value())
            );
            // and the BTC of both exchange should be connected to each other with weight: 1.0
            assert_eq!(
                Some(&1.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_0.get_value(), *node_2.get_value())
            );
            assert_eq!(
                Some(&1.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_2.get_value(), *node_0.get_value())
            );
        }

        #[test]
        fn second_price_update_fetches_first_node_and_inserts_the_second_node() {
            let mut exchange_graph = ExchangeCompleteGraph::default();

            let first_update = PriceUpdate::new(Utc::now(), "KRAKEN", "BTC", "USD", 1000.0, 0.0009);

            let (node_0, node_1) = exchange_graph.add(&first_update);

            let second_update = PriceUpdate::new(Utc::now(), "KRAKEN", "BTC", "EUR", 5000.0, 2.0);

            let (node_2, node_3) = exchange_graph.add(&second_update);

            assert_eq!(3, exchange_graph.index_map.len());

            // both (KRAKEN, BTC) nodes should be with the same Index, but the second should be fetched
            assert_eq!(Index::Inserted(0), node_0);
            assert_eq!(Index::Fetched(0), node_2);
            assert_eq!(node_0.get_value(), node_2.get_value());

            assert_eq!(4, exchange_graph.graph.all_edges().count());
            // first price update edges
            assert_eq!(
                Some(&1000.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_0.get_value(), *node_1.get_value())
            );
            assert_eq!(
                Some(&0.0009),
                exchange_graph
                    .graph
                    .edge_weight(*node_1.get_value(), *node_0.get_value())
            );
            // second price update edges
            assert_eq!(
                Some(&5000.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_2.get_value(), *node_3.get_value())
            );
            assert_eq!(
                Some(&2.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_3.get_value(), *node_2.get_value())
            );
        }

        #[test]
        fn second_price_update_updates_first_node_and_fetches_the_second_node() {
            let mut exchange_graph = ExchangeCompleteGraph::default();

            let first_update = PriceUpdate::new(Utc::now(), "KRAKEN", "BTC", "USD", 1000.0, 0.0009);

            let (node_0, node_1) = exchange_graph.add(&first_update);

            let second_update = PriceUpdate::new(Utc::now(), "KRAKEN", "USD", "ETH", 5000.0, 2.0);

            let (node_2, node_3) = exchange_graph.add(&second_update);

            assert_eq!(3, exchange_graph.index_map.len());

            // both (KRAKEN, USD) nodes should be with the same Index, but the second should be fetched
            assert_eq!(Index::Inserted(1), node_1);
            assert_eq!(Index::Fetched(1), node_2);
            assert_eq!(node_1.get_value(), node_2.get_value());

            assert_eq!(4, exchange_graph.graph.all_edges().count());
            // first price update edges
            assert_eq!(
                Some(&1000.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_0.get_value(), *node_1.get_value())
            );
            assert_eq!(
                Some(&0.0009),
                exchange_graph
                    .graph
                    .edge_weight(*node_1.get_value(), *node_0.get_value())
            );
            // second price update edges
            assert_eq!(
                Some(&5000.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_2.get_value(), *node_3.get_value())
            );
            assert_eq!(
                Some(&2.0),
                exchange_graph
                    .graph
                    .edge_weight(*node_3.get_value(), *node_2.get_value())
            );
        }
    }

    mod index_map {
        use super::*;

        #[test]
        fn creates_entries_and_uses_its_methods_for_fetching() {
            let mut index_hash_map: IndexHashMap<String> = IndexHashMap::new();

            let key_1 = "Key 1".to_string();

            assert_eq!(None, index_hash_map.get(&key_1));
            assert_eq!(false, index_hash_map.contains(&key_1));
            assert_eq!(None, index_hash_map.get_index(&0_usize));
            assert_eq!(false, index_hash_map.contains_index(&0_usize));

            assert_eq!(Index::Inserted(0_usize), index_hash_map.entry(key_1));
            assert_eq!(1_usize, index_hash_map.len());

            // insert second key
            let key_2 = "Key 2".to_string();
            assert_eq!(Index::Inserted(1_usize), index_hash_map.entry(key_2));
            assert_eq!(2_usize, index_hash_map.len());

            let key_1_check = "Key 1".to_string();
            // check for existence of keys
            assert_eq!(Some(&0_usize), index_hash_map.get(&key_1_check));
            assert_eq!(true, index_hash_map.contains(&key_1_check));
            assert_eq!(Some(&key_1_check), index_hash_map.get_index(&0_usize));
            assert_eq!(true, index_hash_map.contains_index(&0_usize));
            assert_eq!(Index::Fetched(0_usize), index_hash_map.entry(key_1_check));
            assert_eq!(2_usize, index_hash_map.len());

            let key_2_check = "Key 2".to_string();
            // check for existence of keys
            let key_2_index_check = 1_usize;

            assert_eq!(Some(&key_2_index_check), index_hash_map.get(&key_2_check));
            assert_eq!(true, index_hash_map.contains(&key_2_check));
            assert_eq!(
                Some(&key_2_check),
                index_hash_map.get_index(&key_2_index_check)
            );
            assert_eq!(true, index_hash_map.contains_index(&key_2_index_check));
            assert_eq!(Index::Fetched(1_usize), index_hash_map.entry(key_2_check));
            assert_eq!(2_usize, index_hash_map.len());
        }
    }
}
