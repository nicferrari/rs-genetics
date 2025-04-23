# rs-genetics
[![Latest version](https://img.shields.io/crates/v/rs-genetics.svg)](https://crates.io/crates/rs-genetics)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/nicferrari/rs-backtester/blob/master/LICENSE-APACHE-2.0)

rs-genetics is a genetic algorithm library written entirely in Rust.

## Get started
- define a fitness function which inputs a Population and outputs an f64 (F:Fn(Population)->f64)
```rust
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
```
- choose an InitializationStrategy
- use the default configuration or change it

```rust
    let init_strategy = InitializationStrategy::F64(Box::new(RandomInitialization));
    let mut config = Config::default();
        config.num_individuals = 1000;
```
- define your Genetic Algorithm and evolve it
```rust
  let mut ga = GA::new(init_strategy,fitness, config)
  let hist = ga.evolve(100);
```
- print the solution
```rust
    let hist = ga.evolve(100);
    let inputs = vec![4.0, -2.0, 3.5, 5.0, -11.0, -4.7];
    let distance: f64 = inputs.iter()
        .zip(&ga.population.get_individual(0).unwrap())
        .map(|(x, y)| x * y)
        .sum();
    println!("Solution = {}",distance);
```
- plot the fitness curve
```rust
    draw_fitness(hist, "fitness_curve.png");
```