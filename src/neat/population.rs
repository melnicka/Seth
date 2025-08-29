// use rand::{seq::IndexedRandom, seq::IndexedMutRandom, Rng};
use std::collections::{HashSet};

use super::genome::*;

pub struct Population {
    pub all_species: Vec<Species>,
    pub pop_size: i32,
    pub current_gen: i32,
}

pub struct Species {
    pub genomes: Vec<Genome>,
    pub average_fitness: f64,
    pub best_fitness: f64,
    pub staleness_counter: i32,
}

impl Population {
    pub fn assign_to_species(&mut self, genome: Genome,
        c1:f64, c2:f64, threshold: f64) {

        for species in &mut self.all_species {
            if same_species(&genome, &species.genomes[0], c1, c2, threshold) {
                species.genomes.push(genome);
                return;
            }
        }
        let new_species = Species{
            genomes: vec![genome],
            average_fitness: 0.0,
            best_fitness: 0.0,
            staleness_counter: 0,
        };

        self.all_species.push(new_species);
    }

}

fn same_species(g1: &Genome, g2: &Genome, c1: f64, c2: f64, threshold: f64) -> bool {
    let dg = detla_genes(g1, g2);
    let dw = delta_weights(g1, g2);
    let delta = c1*dg + c2*dw;
    
    delta < threshold
}

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
