use genetic_algorithms::pop_generic::{GA, InitializationStrategy, RandomInitialization};

fn main() {

    fn fitness(weights:&Vec<f64>)->f64{
        let inputs = vec![4.,-2.,3.5,5.,-11.,-4.7];
        let target = 44.;
        let distance:f64 = inputs.iter().zip(weights.iter()).map(|(x,y)|x*y).collect::<Vec<f64>>().iter().sum();
        return 1./(target-distance).abs();
    }

    let init_strategy = InitializationStrategy::F64(Box::new(RandomInitialization));
    let ga = GA::new(init_strategy);
    ga.inspect();
}