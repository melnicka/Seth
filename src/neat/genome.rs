use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum NodeType {
    Input,
    Hidden,
    Output,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub node_type: NodeType,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct ConnectionID {
    pub in_node_id: u32,
    pub out_node_id: u32,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: ConnectionID,
    pub weight: f64,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct InnovationHistory {
    pub history: HashMap<ConnectionID, u32>,
    pub counter: u32
}

#[derive(Debug)]
pub struct Genome<'a> {
    pub num_inputs: i32,
    pub num_outputs: i32,
    pub total_nodes: i32,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub ih: &'a mut InnovationHistory,
    pub fitness: f64,
}

impl InnovationHistory {
    pub fn get_conn_innovation(&mut self, conn_id: ConnectionID) -> u32 {
        let inno= self.history.get(&conn_id);
        
        match inno {
            Some(i) => return *i,
            None => {
                self.counter +=1;
                let i = self.counter;
                self.history.insert(conn_id, i);
                return i;
            }
        }

    }
}

impl<'a> Genome<'a> {
//     // creates a new network with 
//     pub fn create_new<'a>(num_inputs: i32, 
//         num_outputs: i32, 
//         ih: &'a InnovationHistory) -> Genome<'a> {

//     }

    pub fn add_connection(&mut self, in_node_id: u32, out_node_id: u32, weight: f64) {
        let conn_id = ConnectionID{in_node_id, out_node_id};
        if !self.connection_exist(conn_id) {
            let _inno: u32 = self.ih.get_conn_innovation(conn_id);
            let mut conn = Connection {id: conn_id, weight, enabled: true};
            self.connections.push(conn)
        }
    }

    pub fn connection_exist(&self, conn_id: ConnectionID) -> bool {
        self.connections.iter().any(|c| c.id == conn_id)
    }
}