use std::collections::HashSet;

use petgraph::graphmap::DiGraphMap;

use crate::command::PriceUpdate;
use crate::graph::collection::ExchangeCompleteGraph;
use crate::graph::collection::Index;
use crate::graph::collection::IndexHashMap;

#[derive(Debug)]
pub struct GraphHandler {
    pub exchange_graph: ExchangeCompleteGraph,
    pub price_updates: HashSet<PriceUpdate>,
}

impl Default for GraphHandler {
    fn default() -> Self {
        Self {
            exchange_graph: ExchangeCompleteGraph::default(),
            price_updates: HashSet::default(),
        }
    }
}

impl GraphHandler {
    pub fn handle_update(&mut self, price_update: PriceUpdate) {
        match self.price_updates.get(&price_update) {
            Some(current_price_update) => {
                if price_update.is_eq_and_newer(&current_price_update) {
                    self.exchange_graph.add(&price_update);
                    self.price_updates.replace(price_update);
                }
            }
            None => {
                self.exchange_graph.add(&price_update);
                self.price_updates.insert(price_update);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::prelude::*;

    use super::*;

    #[test]
    fn it_handles_price_update_addition() {
        let mut graph_handler = GraphHandler::default();

        let price_update = PriceUpdate::new(Utc::now(), "Ex", "EUR", "ETH", 1000.0, 000.9);

        assert_eq!(0, graph_handler.price_updates.len());

        graph_handler.handle_update(price_update);

        assert_eq!(1, graph_handler.price_updates.len());
        assert_eq!(2, graph_handler.exchange_graph.get_edges().count());
    }

    #[test]
    fn it_does_not_updates_price_update_with_older_that_the_one_already_present() {
        let mut graph_handler = GraphHandler::default();
        let latest_timestamp = Utc::now();

        let price_update =
            PriceUpdate::new(latest_timestamp.clone(), "Ex", "EUR", "ETH", 1000.0, 0.009);
        graph_handler.handle_update(price_update);

        let older_price_update = PriceUpdate::new(
            Utc.ymd(2017, 12, 10).and_hms(0, 0, 0),
            "Ex",
            "EUR",
            "ETH",
            10.0,
            0.9,
        );
        graph_handler.handle_update(older_price_update);

        assert_eq!(1, graph_handler.price_updates.len());
        assert_eq!(2, graph_handler.exchange_graph.get_edges().count());

        // get the current PriceUpdate
        let current_price_update = graph_handler.price_updates.iter().next().unwrap();

        // we only need to check for the timestamp, forward and backward factors
        assert_eq!(latest_timestamp, current_price_update.timestamp);
        assert_eq!(1000.0, current_price_update.forward_factor);
        assert_eq!(0.009, current_price_update.backward_factor);
    }

    #[test]
    fn it_updates_price_update_that_newer_that_the_one_already_present() {
        let mut graph_handler = GraphHandler::default();
        let latest_timestamp = Utc::now();

        let price_update = PriceUpdate::new(
            Utc.ymd(2017, 12, 10).and_hms(0, 0, 0),
            "Ex",
            "EUR",
            "ETH",
            10.0,
            0.9,
        );
        graph_handler.handle_update(price_update);

        let newer_price_update =
            PriceUpdate::new(latest_timestamp.clone(), "Ex", "EUR", "ETH", 1000.0, 0.009);
        graph_handler.handle_update(newer_price_update);

        assert_eq!(1, graph_handler.price_updates.len());
        assert_eq!(2, graph_handler.exchange_graph.get_edges().count());

        // get the current PriceUpdate
        let current_price_update = graph_handler.price_updates.iter().next().unwrap();

        // we only need to check for the timestamp, forward and backward factors
        assert_eq!(latest_timestamp, current_price_update.timestamp);
        assert_eq!(1000.0, current_price_update.forward_factor);
        assert_eq!(0.009, current_price_update.backward_factor);
    }
}
