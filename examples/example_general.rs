use genetic_algorithms::generalization::Population;
fn main() {
    let population_f64 = Population {
        individuals: vec![
            vec![1.1, 2.2, 3.3],
            vec![4.4, 5.5, 6.6],
        ],
    };

    let population_usize = Population {
        individuals: vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ],
    };

    println!("Population of Vec<f64>:");
    population_f64.inspect();

    println!("Population of Vec<usize>:");
    population_usize.inspect();

    let ga = Population::new(population_f64);
}

