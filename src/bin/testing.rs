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

    let mut _new_pop = Population{all_species: Vec::new(),
        pop_size:0, current_gen:0};

    let g1 = Genome::new(2, 4, &mut dupa);
    let inputs = vec![1.0,2.0];

    print!("{:?}",g1.forward(inputs))
    }
