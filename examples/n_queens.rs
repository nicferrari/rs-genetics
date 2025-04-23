use genetic_algorithms::plot::draw_fitness;
use genetic_algorithms::population::{Config, GA, InitializationStrategy, TSPInitialization, GetPopulation, Population};
fn main() {
    fn calculate_fitness(weights: Population) -> f64 {
        match weights {
            Population::Usize(vec)=>{
            if let Some(individual) = vec.get(0) {
                let mut fitness = 0;
                for i in 0..individual.len() {
                    for j in i + 1..individual.len() {
                        if individual[i] != individual[j] &&
                            individual[i] + i != individual[j] + j &&
                            individual[i] as isize - i as isize != individual[j] as isize - j as isize {
                            fitness += 1;
                        }
                    }
                }
                fitness as f64
            } else {
            0.0
            }
        }
            _ => panic!("Expected Population::usize")
        }
    }
    let num_queens = 10;
    let init_strategy = InitializationStrategy::Usize(Box::new(TSPInitialization));
    let mut config = Config::default();
    config.num_individuals=100;
    config.num_genes = num_queens;
    let mut ga = GA::new(init_strategy, calculate_fitness, config);
    let hist = ga.evolve(100);
    let solution:Vec<usize> = ga.population.get_individual(0).unwrap();
    println!("Solution = {:?}",solution);
    draw_fitness(hist, "fitness_curve.png");
}