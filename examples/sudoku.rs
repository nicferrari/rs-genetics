use genetic_algorithms::population::{TSP};
use genetic_algorithms::GA::GA;

fn main() {
    fn fitness(weights: &Vec<usize>) -> f64 {
        let mut fitness = 0;
        // Check rows
        for row in 0..9 {
            let mut row_count = [0; 9];
            for col in 0..9 {
                let value = weights[row * 9 + col];
                if value != 0 {
                    row_count[(value - 1) as usize] += 1;
                }
            }
            fitness += row_count.iter().filter(|&&count| count > 1).count();
        }
        // Check columns
        for col in 0..9 {
            let mut col_count = [0; 9];
            for row in 0..9 {
                let value = weights[row * 9 + col];
                if value != 0 {
                    col_count[(value - 1) as usize] += 1;
                }
            }
            fitness += col_count.iter().filter(|&&count| count > 1).count();
        }
        // Check subgrids
        for subgrid_row in 0..3 {
            for subgrid_col in 0..3 {
                let mut subgrid_count = [0; 9];
                for row in 0..3 {
                    for col in 0..3 {
                        let value = weights[(subgrid_row * 3 + row) * 9 + subgrid_col * 3 + col];
                        if value != 0 {
                            subgrid_count[(value - 1) as usize] += 1;
                        }
                    }
                }
                fitness += subgrid_count.iter().filter(|&&count| count > 1).count();
            }
        }
        -1.0*fitness as f64
    }
    let population = TSP::initialize(100,81);
    let ga = GA{population, fitness};
    ga.population.inspect();
}
