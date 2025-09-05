use seth::neat::{genome::*, reproduction::*, population::Population, test_utils::*};
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
        pop_size:2, current_gen:0};

    let mut g1 = Genome::new(2, 4, &mut dupa);
    let g2 = Genome::new(2, 4, &mut dupa);
    g1.fitness = 100.0;
    new_pop.assign_to_species(g1, C1, C2, THRESHOLD);
    new_pop.assign_to_species(g2, C1, C2, THRESHOLD);

    }
