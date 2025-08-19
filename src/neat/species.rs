use rand::{seq::IndexedRandom, seq::IndexedMutRandom, Rng};
use std::collections::{HashMap, HashSet};

use super::genome::*;

fn detla_genes(g1: &Genome, g2: &Genome) -> f64 {
    let longer_genome;
    if g1.connections.len() > g2.connections.len() {
        longer_genome = g1.connections.len()
    } else {
        longer_genome = g2.connections.len()
    }
    
    let map1 = g1.get_conn_hashmap();
    let map2 = g2.get_conn_hashmap();
    let keys1: HashSet<ConnectionID> = map1.keys().cloned().collect();
    let keys2: HashSet<ConnectionID> = map2.keys().cloned().collect();
    
    let mismatches: HashSet<ConnectionID> = keys1
    .symmetric_difference(&keys2).cloned().collect();

    (mismatches.len() as f64) / (longer_genome as f64)
}

fn delta_weights(g1: &Genome, g2: &Genome) -> f64 {
    let mut diff = 0.0;

    let map1 = g1.get_conn_hashmap();
    let map2 = g2.get_conn_hashmap();
    let keys1: HashSet<ConnectionID> = map1.keys().cloned().collect();
    let keys2: HashSet<ConnectionID> = map2.keys().cloned().collect();

    let matches: HashSet<ConnectionID> = keys1
    .intersection(&keys2).cloned().collect();

    for key in &matches {
        diff += map1.get(key).unwrap().weight - map2.get(key).unwrap().weight
    }
    (diff / (matches.len() as f64)).abs()
}