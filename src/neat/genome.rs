use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use rand::prelude::IndexedRandom;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Input,
    Hidden,
    Output,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub id: u32,
    pub node_type: NodeType,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct ConnectionID {
    pub in_node_id: u32,
    pub out_node_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Connection {
    pub id: ConnectionID,
    pub weight: f64,
    pub enabled: bool,
}

#[derive(Debug, PartialEq)]
pub struct InnovationHistory {
    pub history: HashMap<ConnectionID, u32>,
    pub counter: u32,
}

#[derive(Debug, PartialEq)]
pub struct Genome {
    pub num_inputs: i32,
    pub num_outputs: i32,
    pub total_nodes: i32,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub fitness: f64,
}

impl InnovationHistory {
    pub fn get_conn_innovation(&mut self, conn_id: ConnectionID) -> u32 {
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
    //     // creates a new network with
    //     pub fn create_new<'a>(num_inputs: i32,
    //         num_outputs: i32,
    //         ih: &'a InnovationHistory) -> Genome<'a> {

    //     }

    pub fn add_connection(&mut self, 
        ih: &mut InnovationHistory, 
        in_node_id: u32, out_node_id:
        u32,
        weight: f64) {
        let conn_id = ConnectionID {
            in_node_id,
            out_node_id,
        };
        if !self.connection_exist(conn_id) {
            let _inno: u32 = ih.get_conn_innovation(conn_id);
            let mut conn = Connection {
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



    pub fn get_valid_node_ids(&self) -> (u32, u32) {
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
}