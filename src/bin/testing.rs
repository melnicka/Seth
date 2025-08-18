use seth::neat::genome::*;
use std::collections::HashMap;


fn main() {
    let n0 = Node{
        id:0,
        node_type: NodeType::Input
    };

    let n1 = Node{
        id:1,
        node_type: NodeType::Input
    };

    let n2 = Node{
        id:2,
        node_type: NodeType::Output
    };

    let c0 = Connection{
        id: ConnectionID { in_node_id: n0.id, out_node_id: n2.id },
        weight: 1.0,
        enabled: true
    };

    let mut dupa = InnovationHistory{
        history: HashMap::new(),
        counter: 0
    };

    let mut g1 = Genome{
        num_inputs: 1,
        num_outputs: 1,
        total_nodes:2,
        nodes: vec![n0.clone(), n1.clone(), n2.clone()],
        connections: Vec::new(),
        fitness: 0.0

    };

    let mut g2 = Genome{
        num_inputs: 1,
        num_outputs: 1,
        total_nodes:2,
        nodes: vec![n0, n1, n2],
        connections: Vec::new(),
        fitness: 0.0

    };

    println!("IH: {:?}", dupa);

    g1.add_connection(&mut dupa, 0, 2, 1.0);

    println!("IH: {:?}", dupa);


    g1.add_connection(&mut dupa, 1, 2, 1.0);

    println!("IH: {:?}", dupa);

    if g1.connection_exist(c0.id) {
        println!("connection exists")
    }

    let inno0 = dupa.get_conn_innovation(g1.connections[0].id);
    println!("c0 innovation number: {}", inno0);

    let inno1 = dupa.get_conn_innovation(g1.connections[1].id);
    println!("c1 innovation number: {}", inno1);

}