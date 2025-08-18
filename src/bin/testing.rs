use seth::neat::genome::*;
use seth::neat::mutations::*;
use std::collections::HashMap;

fn main() {
    let mut dupa = InnovationHistory {
        history: HashMap::new(),
        counter: 0,
    };

    let mut g = Genome::new(1, 2, &mut dupa);
    println!("\n before mutation: {:?}\n",g.connections);
    mutate_weight(&mut g);
    println!("\n after mutation: {:?}\n", g.connections);
}
