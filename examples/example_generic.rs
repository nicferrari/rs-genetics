use genetic_algorithms::population::{Optimization, TSP};
use genetic_algorithms::GA::GA;

fn main() {
    let tsp_population = TSP::initialize(2,5);
    tsp_population.inspect();
    let mut optimization_population = Optimization::initialize(2,5,0.0..1.0);
    optimization_population.inspect();
    optimization_population.update(vec![vec![0.4,-0.1],vec![3.2,2.5]]);
    optimization_population.inspect();
}