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

    let mut new_pop = Population{all_species: Vec::new(),
        pop_size:2, current_gen:0};

    let mut rng = rand::rng();
    let dupa = vec![1];
    let x = dupa.choose(&mut rng).unwrap();
    for _i in 0..3{
        let y = dupa.choose(&mut rng).unwrap();
        if x == y {
            println!("{}", y)
        }
    }
    

    }
