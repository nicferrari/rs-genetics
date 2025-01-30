use genetic_algorithms::population::TSP;
use genetic_algorithms::ga::GA;

fn main() {
    fn calculate_fitness(weights: &Vec<usize>) -> f64 {
        let n = weights.len();
        let mut non_attacking_pairs = n * (n - 1) / 2;

        for i in 0..n {
            for j in (i + 1)..n {
                if (weights[i] as isize - weights[j] as isize).abs() == (i as isize - j as isize).abs() {
                    non_attacking_pairs -= 1;
                }
            }
        }
        non_attacking_pairs as f64
    }

    let n_queens = 10;
    let population = TSP::initialize(100,n_queens);
    let mut ga = GA{population, fitness:&calculate_fitness};
    ga.population.inspect();
    let sorted_pop = ga.evaluate();
    println!("Scores {:?}",sorted_pop.0);
    println!("Sorted individuals {:?}",sorted_pop.1);
    ga.population.update(sorted_pop.1);
    ga.population.inspect();
    println!("{:?}",ga.evolve(1000));
    ga.show_fittest();
}