use rs_genetics::plot::draw_fitness;
use rs_genetics::population::{Config, GA, GetPopulation, InitializationStrategy, Population, TSPInitialization};

fn main() {
    struct City{x:f64,y:f64,};
    fn distance_(city1: &City, city2: &City) -> f64 {
        ((city1.x - city2.x).powi(2) + (city1.y - city2.y).powi(2)).sqrt()
    }
    fn total_distance(weights: Population) -> f64 {
        //solving for 5-points star
        //historical europe tour: Rome, Paris, Athens, Berlin, London, Vienna
        let cities = vec![City{x:41.9028,y:12.4964},City{x:48.8566,y:2.3522},City{x:37.9838,y:23.7275},City{x:52.5200,y:13.4050},City{x:51.5074,y:-0.1278},City{x:48.2082,y:16.3738}];
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
    config.num_individuals=100;
    config.num_genes = 6;
    let mut ga = GA::new(init_strategy, total_distance, config);

    let hist = ga.evolve(100);
    let solution:Vec<usize> = ga.population.get_individual(0).unwrap();
    println!("Solution = {:?}",solution);
    draw_fitness(hist, "fitness_curve.png");
}