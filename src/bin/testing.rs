use seth::neat::genome::*;
use seth::neat::mutations::*;
use std::collections::HashMap;

fn main() {
    let mut dupa = InnovationHistory {
        history: HashMap::new(),
        counter: 0,
    };

    let parent1 = Genome::new(1, 2, &mut dupa);
    let parent2 = Genome::new(1, 2, &mut dupa);
    println!("\n parent1: {:?}\n",parent1);
    println!("\n parent2: {:?}\n", parent2);
}
