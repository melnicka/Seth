use seth::neat::genome::*;
use std::collections::HashMap;

fn main() {
    let n0 = Node {
        id: 0,
        node_type: NodeType::Input,
    };

    let n1 = Node {
        id: 1,
        node_type: NodeType::Input,
    };

    let n2 = Node {
        id: 2,
        node_type: NodeType::Output,
    };

    let c0 = Connection {
        id: ConnectionID {
            in_node_id: n0.id,
            out_node_id: n2.id,
        },
        weight: 1.0,
        enabled: true,
    };

    let mut dupa = InnovationHistory {
        history: HashMap::new(),
        counter: 0,
    };

    let mut g1 = Genome {
        num_inputs: 1,
        num_outputs: 1,
        total_nodes: 2,
        nodes: vec![n0.clone(), n1.clone(), n2.clone()],
        connections: Vec::new(),
        fitness: 0.0,
    };

    let mut g2 = Genome {
        num_inputs: 1,
        num_outputs: 1,
        total_nodes: 2,
        nodes: vec![n0, n1, n2],
        connections: Vec::new(),
        fitness: 0.0,
    };

    let (id1, id2) = g1.get_valid_node_ids();
    println!("{}, {}", id1, id2)
}
