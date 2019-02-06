use crate::graph::handler::GraphHandler;
use std::collections::HashMap;
use petgraph::graphmap::AllEdges;
use petgraph::Directed;

fn best(graph_handler: &GraphHandler) {
}

fn initialize_rates(edges: AllEdges<usize, f64, Directed>) -> HashMap<usize, HashMap<usize, f64>> {
    edges.fold(HashMap::new(), |mut acc, (from_node, to_node, _weight)| {
        let mut dest_hash_map = HashMap::new();
        dest_hash_map.insert(to_node, 0.0);

        acc.insert(from_node, dest_hash_map);
        acc
    })
}

fn initialize_next(edges: AllEdges<usize, f64, Directed>) -> HashMap<usize, HashMap<usize, Option<usize>>> {
    edges.fold(HashMap::new(), |mut acc, (from_node, to_node, _weight)| {
        let mut dest_hash_map = HashMap::new();
        dest_hash_map.insert(to_node, None);

        acc.insert(from_node, dest_hash_map);
        acc
    })
}

/// TODO: Make sure this is tested!
fn best_rates(graph_handler: &GraphHandler) -> HashMap<usize, HashMap<usize, Option<usize>>> {
    let mut rate = initialize_rates(graph_handler.exchange_graph.graph.all_edges());
    let mut next = initialize_next(graph_handler.exchange_graph.graph.all_edges());

    let v = &graph_handler.exchange_graph.index_map;

    for (node_1, node_2, &weight) in graph_handler.exchange_graph.graph.all_edges() {
        let mut rate_entry = rate.entry(node_1).or_insert(HashMap::new());
        rate_entry.insert(node_2, weight);

        let mut next_entry = next.entry(node_1).or_insert(HashMap::new());
        next_entry.insert(node_2, Some(node_2));
    }

    for k in 0..v.len() {
        for i in 0..v.len() {
            for j in 0..v.len() {
                let mul_rate = rate.get(&i).unwrap().get(&k).unwrap() * rate.get(&k).unwrap().get(&j).unwrap();

                if *rate.get(&i).unwrap().get(&j).unwrap() < mul_rate {

                    rate.get_mut(&i).unwrap().insert(j, mul_rate);
                    let next_i = next.get_mut(&i).unwrap();
                    let next_record = next_i.get(&k).unwrap();

                    next_i.insert(j, *next_record);
                }

            }
        }
    }
    next
}

#[cfg(test)]
mod test {
    #[test]
    fn it_inserts_rate() {

    }
}
