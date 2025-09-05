use rand::Rng;
use std::collections::{HashMap};
use rand::prelude::IndexedRandom;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Input,
    Hidden,
    Output,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: i32,
    pub node_type: NodeType,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct ConnectionID {
    pub in_node_id: i32,
    pub out_node_id: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Connection {
    pub id: ConnectionID,
    pub weight: f64,
    pub enabled: bool,
}

#[derive(Debug, PartialEq)]
pub struct InnovationHistory {
    pub history: HashMap<ConnectionID, i32>,
    pub counter: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Genome {
    pub num_inputs: i32,
    pub num_outputs: i32,
    pub total_nodes: i32,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub fitness: f64,
    pub adjusted_fitness: f64
}

impl InnovationHistory {
    pub fn get_conn_innovation(&mut self, conn_id: ConnectionID) -> i32 {
        let inno = self.history.get(&conn_id);

        match inno {
            Some(i) => return *i,
            None => {
                self.counter += 1;
                let i = self.counter;
                self.history.insert(conn_id, i);
                return i;
            }
        }
    }
}

impl Genome {
         pub fn new(num_inputs: i32,
            num_outputs: i32,
            ih: &mut InnovationHistory) -> Genome {

            let mut in_node_ids:Vec<i32> = Vec::new();
            let mut out_node_ids:Vec<i32> = Vec::new();

            let mut rng = rand::rng();

            let mut g = Genome {
                num_inputs,
                num_outputs,
                total_nodes: 0,
                nodes: Vec::new(),
                connections: Vec::new(),
                fitness: 0.0,
                adjusted_fitness: 0.0
            };

            for _i in 0..num_inputs {
                in_node_ids.push(g.total_nodes);
                g.add_node(g.total_nodes, NodeType::Input);
            };

            for _i in 0..num_outputs {
                out_node_ids.push(g.total_nodes);
                g.add_node(g.total_nodes, NodeType::Output);
            };

            for in_id in &in_node_ids {
                for out_id in &out_node_ids {
                    let weight: f64 = rng.random_range(-2.0..2.0);
                    g.add_connection(ih, *in_id, *out_id, weight);
                };
            };
            g
         }

         pub fn forward(&self, inputs: Vec<f64>) -> Vec<f64> {
            let mut node_values: HashMap<i32, f64> = HashMap::new();
            let mut node_sums: HashMap<i32, f64> = HashMap::new();
            let mut input_index = 0;
            let mut outputs: Vec<f64> = Vec::new();
        
            for node in &self.nodes {
                match node.node_type{
                    NodeType::Input => {
                        if input_index < inputs.len() {
                            node_values.insert(node.id, inputs[input_index]);
                            input_index +=1;
                        }
                    },
                    _ => {
                        node_values.insert(node.id, 0.0);
                    },
                }
            }

            for conn in &self.connections{
                if conn.enabled {
                    let node_value = node_values.get(&conn.id.in_node_id).unwrap();
                    let sum = node_sums.entry(conn.id.out_node_id).or_insert(0.0);
                    *sum += node_value * conn.weight;
                }
            }
            
            for node in &self.nodes{
                match node.node_type {
                    NodeType::Output => {
                        let val:&mut f64 = node_values.get_mut(&node.id).unwrap();
                        *val = sigmoid(*node_sums.get(&node.id).unwrap());
                    }
                    _ => {
                        let val = node_values.get_mut(&node.id).unwrap();
                        *val = relu(*node_sums.get(&node.id).unwrap_or(&0.0));
                    }
                }
            }

            for node in &self.nodes {
                match node.node_type {
                    NodeType::Output => {
                        outputs.push(*node_values.get(&node.id).unwrap());
                    },
                    _ => {}
                }
            }

            outputs
        }

    pub fn calculate_fitness(&mut self, score: i32, time_survived: f64) {
        let fitness = (score as f64) * 100.0 + time_survived;
        self.fitness = fitness;
    }

    pub fn add_node(&mut self, node_id: i32, node_type: NodeType) {
        let new_node = Node {
            id: node_id,
            node_type
        };
        self.nodes.push(new_node);
        self.total_nodes +=1;
    }

    pub fn add_connection(&mut self, 
        ih: &mut InnovationHistory, 
        in_node_id: i32, out_node_id:
        i32,
        weight: f64) {
        let conn_id = ConnectionID {
            in_node_id,
            out_node_id,
        };
        if !self.connection_exist(conn_id) {
            let _inno: i32 = ih.get_conn_innovation(conn_id);
            let conn = Connection {
                id: conn_id,
                weight,
                enabled: true,
            };
            self.connections.push(conn)
        }
    }

    pub fn connection_exist(&self, conn_id: ConnectionID) -> bool {
        self.connections.iter().any(|c| c.id == conn_id)
    }



    pub fn get_valid_node_ids(&self) -> (i32, i32) {
        let mut rng = rand::rng();

        loop {
            let n1 = self.nodes.choose(&mut rng).unwrap();
            let n2 = self.nodes.choose(&mut rng).unwrap();

            match (&n1.node_type, &n2.node_type) {
                (NodeType::Input, NodeType::Hidden)
                | (NodeType::Input, NodeType::Output)
                | (NodeType::Hidden, NodeType::Output)
                | (NodeType::Hidden, NodeType::Hidden) => return (n1.id, n2.id),

                (NodeType::Output, NodeType::Input) => return (n2.id, n1.id),

                _ => continue,
            }
        }
    }

    pub fn get_conn_hashmap(&self) -> HashMap<ConnectionID, &Connection> {
        let mut conn_map: HashMap<ConnectionID, &Connection> = HashMap::new();
        for conn in &self.connections {
            conn_map.insert(conn.id, conn);
        };
        conn_map
    }
}


fn relu(x: f64) -> f64 {
    if x > 0.0 {
        return x;
    } 
    return 0.0;
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}
