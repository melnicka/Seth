use std::collections::HashMap;

pub enum NodeType {
    Input,
    Hidden,
    Output,
}

pub struct Node {
    ID: u32,
    node_type: NodeType,
}

pub struct ConnectionID {
    in_node_id: u32,
    out_node_id: u32,
}

pub struct Connection {
    ID: ConnectionID,
    weight: f64,
    enabled: bool,
}

pub struct InnovationHistory {
    history: HashMap<ConnectionID, u32>,
    counter: i32
}

pub struct Genome {
    num_inputs: i32,
    num_outputs: i32,
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    IH: &InnovationHistory,
    fitness: i32,
}
