use rs_genetics::plot::draw_fitness;
use rs_genetics::population::{Config, GA, InitializationStrategy, TSPInitialization, GetPopulation, Population, SudokuInitialization};
fn main() {
    fn calculate_fitness(weights:Population) -> f64 {
        match weights {
            Population::Usize(vec)=>{
                let mut score = 0.0;
                // Check rows
                for row in 0..9 {
                    let start = row * 9;
                    let unique_digits: std::collections::HashSet<_> =
                        vec[0][start..start + 9].iter().cloned().collect();
                    score += unique_digits.len() as f64;
                }
                // Check columns
                for col in 0..9 {
                    let unique_digits: std::collections::HashSet<_> =
                        (0..9).map(|row| vec[0][row * 9 + col]).collect();
                    score += unique_digits.len() as f64;
                }
                // Check subgrids
                for subgrid_row in (0..9).step_by(3) {
                    for subgrid_col in (0..9).step_by(3) {
                        let mut unique_digits = std::collections::HashSet::new();
                        for i in 0..3 {
                            for j in 0..3 {
                                unique_digits.insert(vec[0][(subgrid_row + i) * 9 + subgrid_col + j]);
                            }
                        }
                        score += unique_digits.len() as f64;
                    }
                }
                // Penalize invalid numbers (outside 1-9)
                for &value in vec[0].iter() {
                    if value < 1 || value > 9 {
                        score -= 5.0; // Arbitrary penalty
                    }
                }
                // Add bonus for completeness
                let filled_cells = vec[0].iter().filter(|&&value| value != 0).count();
                score += filled_cells as f64 / 81.0; // Fraction of cells filled
                score
            }
            _ => panic!("Expected Population::usize")
        }
    }
    let init_strategy = InitializationStrategy::Usize(Box::new(SudokuInitialization));
    let mut config = Config::default();
    config.num_individuals=100;
    config.num_genes = 81;
    let mut ga = GA::new(init_strategy, calculate_fitness, config);
    ga.inspect();
    //ga.evaluate();
    let hist = ga.evolve(100);
    let solution:Vec<usize> = ga.population.get_individual(0).unwrap();
    println!("Solution = {:?}",solution);
    draw_fitness(hist, "fitness_curve.png");
}
