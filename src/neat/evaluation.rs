use super::genome::Genome;

pub fn evaluate_fitness(genome: &mut Genome, time_survived: f64, score:i32) {
    let fitness = 100.0 * (score as f64) + time_survived;
    genome.fitness = fitness;
}