use rand::Rng;
use std::collections::HashMap;

use super::genome::*;

pub fn mutate_weight(genome: &mut Genome) {
    let mut rng = rand::rng();

    for conn in &mut genome.connections {
        if rng.random_range(0.0..1.0) < 0.8 {
            let delta = rng.random_range(-0.2..0.2);
            conn.weight += delta;
        } else {
            conn.weight = rng.random_range(-1.0..1.0)
        }
    }
    
}