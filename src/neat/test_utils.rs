use super::{genome::*, population::*};

pub fn display_genome(genome: &Genome) {
    println!("\nTotal nodes: {:?}",genome.total_nodes);
    println!("Node ids:");
    for node in &genome.nodes {
        print!("{:?} ", node.id);
    }
    println!("\nConnections:");
    for conn in &genome.connections {
        println!("Conn id: (in_node:{:?} out_node:{:?}) Conn weight: {:?} Conn enabled: {:?}", 
    conn.id.in_node_id, conn.id.out_node_id, conn.weight, conn.enabled)
    }
}

pub fn display_species(species: &Species) {
    println!("\nNumer of genomes:{:?}", species.genomes.len());
    println!("Average fitness:{:?}", species.average_fitness);
    println!("Breedinf rate:{:?}", species.breeding_rate);
}

pub fn display_population(population: &Population) {
    println!("\nNumber of species:{:?}", population.all_species.len());
    println!("Population size:{:?}", population.pop_size);
    println!("Current generation:{:?}", population.current_gen)
}