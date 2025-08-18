use seth::neat::genome::*;
use std::collections::HashMap;

fn main() {
    let mut dupa = InnovationHistory {
        history: HashMap::new(),
        counter: 0,
    };

    let g = Genome::new(1, 2, &mut dupa);
    println!("{:?}", g)
 
}
