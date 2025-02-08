use genetic_algorithms::pop_generic::{Config, GA, InitializationStrategy, RandomInitialization};
use genetic_algorithms::pop_generic::Population;

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
    /*
    ga.inspect();
    let evals = ga.evaluate();
    println!("evaluations = {:?}",evals);
    ga.sort(evals);
    println!("new evaluations = {:?}",ga.evaluate());
    let selected = ga.rank_selection_cdf();
    println!("selected parents = {:?}",selected);
    ga.update(selected);
    ga.inspect();
    println!("xover between individual 1 and 2 = {:?}",ga.population.crossover(1,2));
    let mated_pop = ga.mate_population();
    ga.update(mated_pop);
    ga.inspect();
    println!("_________________");
    let mutated_pop = ga.mutate();
    ga.update(mutated_pop);
    ga.inspect();*/
    ga.evolve(100);
    let inputs = vec![4.0, -2.0, 3.5, 5.0, -11.0, -4.7];
    let distance: f64 = inputs.iter()
        .zip(&ga.population.get_individuals().unwrap()[0])
        .map(|(x, y)| x * y)
        .sum();
    println!("Solution = {}",distance);
}