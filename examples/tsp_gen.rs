use genetic_algorithms::pop_generic::{Config, GA, InitializationStrategy, Population, TSPInitialization};

fn main() {
    struct City{x:f64,y:f64,};
    fn distance_(city1: &City, city2: &City) -> f64 {
        ((city1.x - city2.x).powi(2) + (city1.y - city2.y).powi(2)).sqrt()
    }
    fn total_distance(weights: Population) -> f64 {
        //solving for 5-points star
        let cities = vec![City{x:0.,y:0.},City{x:1.,y:2.},City{x:4.,y:1.},City{x:2.,y:-1.},City{x:1.,y:-2.}];
        let mut distance = 0.0;
        match weights {
            Population::Usize(vec)=>{
            for i in 0..cities.len() - 1 {
                distance += distance_(&cities[vec[0][i]], &cities[vec[0][i + 1]]);
            }
            distance += distance_(&cities[cities.len() - 1], &cities[0]);
            distance
            }
            _ => panic!("Expected Population::usize")
        }
    }

    let init_strategy = InitializationStrategy::Usize(Box::new(TSPInitialization));
    let mut config = Config::default();
    config.num_genes = 5;
    let mut ga = GA::new(init_strategy, total_distance, config);
    ga.inspect();
    let evals = ga.evaluate();
    println!("evaluations = {:?}",evals);
    ga.sort(evals);
    println!("new evaluations = {:?}",ga.evaluate());
    let selected = ga.rank_selection_cdf();
    println!("selected parents = {:?}",selected);
    ga.update(selected);
    ga.inspect();
    //println!("xover between individual 1 and 2 = {:?}",ga.population.crossover(1,2));
}