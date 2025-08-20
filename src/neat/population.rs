use rand::{seq::IndexedRandom, seq::IndexedMutRandom, Rng};
use std::collections::{HashMap, HashSet};

use super::genome::*;

pub struct Population {
    pub all_species: Vec<Species>,
    pub pop_size: i32,
    pub current_gen: i32,
    species_threshold: f64,
    c1: f64, // delta genes
    c2: f64 // delta weights
}

pub struct Species {
    pub genomes: Vec<Genome>,
    pub average_fitness: f64,
    pub breeding_rate: i32
}

impl Population {
    pub fn assign_to_species(&mut self, genome: Genome) {
        for species in &mut self.all_species {
            if self.same_species(&genome, &species.genomes[0]) {
                species.genomes.push(genome);
                return;
            }
        }
        let mut new_species = Species{
            genomes: vec![genome],
            average_fitness: 0.0,
            breeding_rate: 0
        };

        self.all_species.push(new_species);
    }

    pub fn same_species(&self, g1: &Genome, g2: &Genome) -> bool {
        let dg = detla_genes(g1, g2);
        let dw = delta_weights(g1, g2);
        let delta = self.c1*dg + self.c2*dw;
        
        delta < self.species_threshold
    }
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