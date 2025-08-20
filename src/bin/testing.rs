use seth::neat::{genome::*, mutations::*, population::Population, test_utils::*};
use std::collections::HashMap;
const C1: f64 = 1.0;
const C2: f64 = 1.0;
const THRESHOLD: f64 = 1.0;

fn main() {    
    let mut dupa = InnovationHistory {
        history: HashMap::new(),
        counter: 0,
    };

    let mut new_pop = Population{all_species: Vec::new(),
        pop_size:0, current_gen:0};

    let mut parent1 = Genome::new(1, 3, &mut dupa);
    parent1.fitness = 100.0;
    let parent2 = Genome::new(1, 2, &mut dupa);
    let offspring = crossover(&parent1, &parent2);

    display_population(&new_pop);
    new_pop.assign_to_species(parent1, C1, C2, THRESHOLD);
    display_population(&new_pop);
    new_pop.assign_to_species(parent2, C1, C2, THRESHOLD);
    display_population(&new_pop);
    new_pop.assign_to_species(offspring, C1, C2, THRESHOLD);
    display_population(&new_pop);

    }
