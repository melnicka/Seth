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

    let mut biskupa = Genome{
        num_inputs: 1,
        num_outputs: 1,
        total_nodes:2,
        nodes: vec![n0, n1, n2],
        connections: Vec::new(),
        ih: &mut dupa,
        fitness: 0.0

    };

    println!("IH: {:?}", biskupa.ih);

    biskupa.add_connection(0, 2, 2.0);

    println!("IH: {:?}", biskupa.ih);


    biskupa.add_connection(1, 2, 2.0);

    println!("IH: {:?}", biskupa.ih);

    if biskupa.connection_exist(c0.id) {
        println!("connection exists")
    }

    let inno0 = biskupa.ih.get_conn_innovation(biskupa.connections[0].id);
    println!("c0 innovation number: {}", inno0);

    let inno1 = biskupa.ih.get_conn_innovation(biskupa.connections[1].id);
    println!("c1 innovation number: {}", inno1);

}