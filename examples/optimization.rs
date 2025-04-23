use rs_genetics::plot;
use rs_genetics::population::{Config, GA, InitializationStrategy, RandomInitialization, GetPopulation};
use rs_genetics::population::Population;
use plot::draw_fitness;

fn main() {
    fn fitness(weights: Population) -> f64 {
        let inputs = vec![4.0, -2.0, 3.5, 5.0, -11.0, -4.7];
        let target = 44.0;
        match weights {
            Population::F64(vec) => {
                let distance: f64 = inputs.iter()
                    .zip(&vec[0])
                    .map(|(x, y)| x * y)
                    .sum();
                let result = 1.0 / ((target - distance).abs()+0.000000001);
                result
            }
            _ => panic!("Expected Population::F64"),
        }
    }

    let init_strategy = InitializationStrategy::F64(Box::new(RandomInitialization));
    let mut config = Config::default();
    config.num_individuals = 1000;
    let mut ga = GA::new(init_strategy,fitness, config);

    let hist = ga.evolve(100);
    let inputs = vec![4.0, -2.0, 3.5, 5.0, -11.0, -4.7];
    let distance: f64 = inputs.iter()
        .zip(&ga.population.get_individual(0).unwrap())
        .map(|(x, y)| x * y)
        .sum();
    println!("Solution = {}",distance);
    draw_fitness(hist, "fitness_curve.png");
}