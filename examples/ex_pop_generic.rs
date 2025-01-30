use genetic_algorithms::pop_generic::{InitializationStrategy,RandomInitialization,TSPInitialization,GA};

fn main() {
    let init_strategy:InitializationStrategy = InitializationStrategy::Usize(Box::new(TSPInitialization));
    let ga = GA::new(init_strategy);
    ga.inspect();
    let init_strategy2:InitializationStrategy = InitializationStrategy::F64(Box::new(RandomInitialization));
    let ga2 = GA::new(init_strategy2);
    ga2.inspect();
}