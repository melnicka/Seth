use rand::{seq::IndexedRandom, Rng};
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

    let mut new_pop = Population::initialize_pop(
    3,&mut dupa, 2, 2);
    new_pop.new_generation(0.6, &mut dupa, C1, C2, THRESHOLD);

    println!("{:?}", new_pop)
    
    }
