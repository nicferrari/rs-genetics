use genetic_algorithms::pop_generic::{InitializationStrategy,RandomInitialization,GA};

fn main() {
    let init_strategy:InitializationStrategy = InitializationStrategy::Usize(Box::new(RandomInitialization));
    let ga = GA::new(init_strategy);
    ga.run();
    let init_strategy2:InitializationStrategy = InitializationStrategy::F64(Box::new(RandomInitialization));
    let ga2 = GA::new(init_strategy2);
    ga2.run();
}