use genetic_algorithms::GA::GA;
use genetic_algorithms::population::{Crossover, TSP};

fn main() {
    let population = TSP::initialize(10,5);
    struct City{x:f64,y:f64,};
    fn distance_(city1: &City, city2: &City) -> f64 {
        ((city1.x - city2.x).powi(2) + (city1.y - city2.y).powi(2)).sqrt()
    }
    fn total_distance(weights: &Vec<usize>) -> f64 {
        //solving for 5-points star
        let cities = vec![City{x:0.,y:0.},City{x:1.,y:2.},City{x:4.,y:1.},City{x:2.,y:-1.},City{x:1.,y:-2.}];
        let mut distance = 0.0;
        for i in 0..cities.len() - 1 {
            distance += distance_(&cities[weights[i]], &cities[weights[i + 1]]);
        }
        distance += distance_(&cities[cities.len() - 1], &cities[0]);
        distance
    }

    let mut ga = GA{population, fitness:&total_distance};
    let sorted_pop = ga.evaluate();
    println!("Scores {:?}",sorted_pop.0);
    println!("Sorted individuals {:?}",sorted_pop.1);
    ga.population.update(sorted_pop.1);
    println!("Individual 0 eval {:?}",ga.evaluate_individual(0));
    let selected_pop = ga.population.rank_selection_cumulative_distr();
    ga.population.update(selected_pop);
    ga.population.inspect();
    println!("{:?}",ga.population.crossover(0,1));
}