// use rand::{seq::IndexedRandom, seq::IndexedMutRandom, Rng};
use std::collections::{HashSet};

use rand::{seq::IndexedRandom, Rng};

use crate::neat::reproduction::{crossover, mutate_add_conn, mutate_add_node, mutate_toggle_connection, mutate_weight};

use super::genome::*;

#[derive(Debug)]
pub struct Population {
    pub all_species: Vec<Species>,
    pub pop_size: i32,
    pub current_gen: i32,
}

#[derive(Debug)]
pub struct Species {
    pub genomes: Vec<Genome>,
    pub average_fitness: f64,
    pub best_fitness: f64,
}

impl Population {
    pub fn new_generation(&mut self, keep_ratio: f64, ih: &mut InnovationHistory,
    c1: f64, c2:f64, threshold: f64) {
        let mut new_genomes:Vec<Genome>= Vec::new();
        let mut total_fitness: f64 = 0.0;
        let mut rng = rand::rng();

        self.kill_weakest_genomes(keep_ratio);

        // calculating species average fitness and total fitness
        for species in &mut self.all_species{
            species.calculate_average_fitness();
            total_fitness += species.average_fitness;
        }

        // reproduction 
        for species in &mut self.all_species{
            let offspring_count: i32 = ((species.average_fitness/total_fitness)as i32)*self.pop_size;

            for _i in 1..offspring_count{
                let parent1 = species.parent_selection(3);
                let parent2 = species.parent_selection(3);
                let mut offspring = crossover(parent1, parent2);

                if rng.random_range(0.0..1.0) < 0.5 {
                    mutate_add_conn(&mut offspring, ih);
                }
                if rng.random_range(0.0..1.0) < 0.6 {
                    mutate_weight(&mut offspring);
                }
                if rng.random_range(0.0..1.0) < 0.2 {
                    mutate_add_node(&mut offspring, ih);
                }
                if rng.random_range(0.0..1.0) < 0.1{
                    mutate_toggle_connection(&mut offspring);
                }

                new_genomes.push(offspring);
            }

            // always keeping copies of the best genomes from each species
            let champion = species.find_champion().clone();
            new_genomes.push(champion)
        }

        // assigning new population
        self.current_gen += 1;
        self.all_species = Vec::new();
        for genome in new_genomes{
            self.assign_to_species(genome, c1, c2, threshold);
        }
        
    }

    pub fn initialize_pop(pop_size: i32, ih: &mut InnovationHistory,
    in_layer_nodes: i32, out_layer_nodes: i32) -> Population{
        let mut pop = Population{
            all_species: Vec::new(),
            pop_size,
            current_gen: 0
        };

        let mut starting_species = Species{
            genomes: Vec::new(),
            average_fitness: 0.0,
            best_fitness:0.0
        };
        
        for _i in 0..pop_size{
            let genome = Genome::new(in_layer_nodes, 
                out_layer_nodes, ih);
            starting_species.genomes.push(genome);
        }
        pop.all_species.push(starting_species);
        pop
    }

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
        };

        self.all_species.push(new_species);
    }

    pub fn kill_weakest_genomes(&mut self, keep_ratio: f64) {
        let mut all_genomes: Vec<(usize, Genome)> = Vec::new();

        // calculating adjusted fitness
        for (i, species) in self.all_species.iter_mut().enumerate(){
            let species_size = species.genomes.len() as f64;
            for genome in &mut species.genomes {
                genome.adjusted_fitness = genome.fitness /species_size;
            }
            all_genomes.extend(species.genomes.drain(..)
            .map(|g| (i, g)))
        }
        all_genomes.sort_by(|a, b|
        b.1.adjusted_fitness.partial_cmp(&a.1.adjusted_fitness).unwrap());
        
        let keep_count = ((self.pop_size as f64) * keep_ratio).ceil() as usize;
        all_genomes.truncate(keep_count);

        for (i, genome) in all_genomes{
            self.all_species[i].genomes.push(genome);
        }
        
        self.all_species.retain(|sp| !sp.genomes.is_empty());
    }

}

impl Species {
    pub fn calculate_average_fitness(&mut self) {
        let mut sum = 0.0;
        for genome in &self.genomes {
            sum += genome.fitness;
        }
        self.average_fitness = sum / (self.genomes.len() as f64);
    }

    pub fn find_champion(&mut self) -> &Genome {
        let mut champion = &self.genomes[0];
        for genome in &self.genomes{
            if genome.fitness > champion.fitness{
                champion = genome;
            }
        }
        self.best_fitness = champion.fitness;
        champion
    }

    pub fn parent_selection(&self, k: i32) -> &Genome {
        let mut rng =  rand::rng();
        let mut best = self.genomes.choose(&mut rng).unwrap();

        for _i in 1..k {
            let new = self.genomes.choose(&mut rng).unwrap();
            if new.fitness > best.fitness {
                best = new;
            }
        }
        best
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
