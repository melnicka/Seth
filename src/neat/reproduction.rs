use rand::{seq::IndexedMutRandom, Rng};
use std::collections::{HashSet};

use super::genome::*;

pub fn crossover(parent1: &Genome, parent2: &Genome) -> Genome {
    let mut rng = rand::rng();

    let mut fitter_parent = parent1;
    let mut less_fit_parent = parent2;
    if parent1.fitness < parent2.fitness {
        fitter_parent = parent2;
        less_fit_parent = parent1;
    }

    let map1 = fitter_parent.get_conn_hashmap();
    let map2 = less_fit_parent.get_conn_hashmap();
    let keys1: HashSet<ConnectionID> = map1.keys().cloned().collect();
    let keys2: HashSet<ConnectionID> = map2.keys().cloned().collect();
    let matches: HashSet<ConnectionID> = keys1
    .intersection(&keys2).cloned().collect();
    let mismatches: HashSet<ConnectionID> = keys1
    .difference(&keys2).cloned().collect();

    let mut offspring = Genome {
        num_inputs: fitter_parent.num_inputs,
        num_outputs: fitter_parent.num_outputs,
        total_nodes: fitter_parent.total_nodes,
        nodes: fitter_parent.nodes.clone(),
        connections: Vec::new(),
        fitness: 0.0
    };

    for conn_id in matches {
        if rng.random_range(0.0..1.0) > 0.5 {
            let new_conn = *map1.get(&conn_id).unwrap();
            offspring.connections.push(new_conn.clone());
        } else {
            let new_conn = *map2.get(&conn_id).unwrap();
            offspring.connections.push(new_conn.clone());
        }
    }

    for conn_id in mismatches {
        let new_conn = *map1.get(&conn_id).unwrap();
        offspring.connections.push(new_conn.clone())
    }
    offspring
}


pub fn mutate_weight(genome: &mut Genome) {
    let mut rng = rand::rng();

    for conn in &mut genome.connections {
        if rng.random_range(0.0..1.0) < 0.8 {
            let delta = rng.random_range(-0.2..0.2);
            conn.weight += delta;
        } else {
            conn.weight = rng.random_range(-2.0..2.0)
        }
    }
    
 
}

pub fn mutate_add_conn(genome: &mut Genome, ih: &mut InnovationHistory) {
    let mut rng = rand::rng(); 

    let (in_node_id, out_node_id) = genome.get_valid_node_ids();
    let weight = rng.random_range(-2.0..2.0);
    genome.add_connection(ih, in_node_id, out_node_id, weight);
}

pub fn mutate_add_node(genome: &mut Genome, ih: &mut InnovationHistory) {
    let mut rng = rand::rng();

    if let Some(conn) = genome.connections.choose_mut(&mut rng) {
        conn.enabled = false;
        let n_in = conn.id.in_node_id;
        let n_out = conn.id.out_node_id;

        let n_new = genome.total_nodes;
        genome.add_node(n_new, NodeType::Hidden);
        let mut weight = rng.random_range(-2.0..2.0);
        genome.add_connection(ih, n_in, n_new, weight);
        weight = rng.random_range(-2.0..2.0);
        genome.add_connection(ih, n_new, n_out, weight);
        
    }

}

pub fn mutate_toggle_connection(genome: &mut Genome) {
    let mut rng = rand::rng();
    if let Some(conn) = genome.connections.choose_mut(&mut rng) {
        conn.enabled = !conn.enabled;
    }
}
