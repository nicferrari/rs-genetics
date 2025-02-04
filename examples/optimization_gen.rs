use genetic_algorithms::pop_generic::{GA, InitializationStrategy, RandomInitialization};
use genetic_algorithms::pop_generic::Population;
use genetic_algorithms::pop_generic::Crossover;

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
                1.0 / (target - distance).abs()
            }
            _ => panic!("Expected Population::F64"),
        }
    }

    let init_strategy = InitializationStrategy::F64(Box::new(RandomInitialization));
    let mut ga = GA::new(init_strategy,fitness);
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
}