use genetic_algorithms::population::{Crossover,MutatePopulationOptimization, Optimization, Population};
use genetic_algorithms::GA::{GA};
/*Given the following function:
y = f(w1:w6) = w1x1 + w2x2 + w3x3 + w4x4 + w5x5 + 6wx6
where (x1,x2,x3,x4,x5,x6)=(4,-2,3.5,5,-11,-4.7) and y=44
What are the best values for the 6 weights (w1 to w6)? We are going to use the genetic algorithm to optimize this function.
*/
fn main() {
    let population = Optimization::initialize(100,6,-10.0..10.0);
    fn fitness(weights:&Vec<f64>)->f64{
        let inputs = vec![4.,-2.,3.5,5.,-11.,-4.7];
        let target = 44.;
        let distance:f64 = inputs.iter().zip(weights.iter()).map(|(x,y)|x*y).collect::<Vec<f64>>().iter().sum();
        return 1./(target-distance).abs();
    }
    let mut ga = GA{population, fitness:&fitness};
    let sorted_pop = ga.evaluate();
    println!("Scores {:?}",sorted_pop.0);
    println!("Sorted individuals {:?}",sorted_pop.1);
    ga.population.update(sorted_pop.1);
    println!("Individual 0 eval {:?}",ga.evaluate_individual(0));
    let selected_pop = ga.population.rank_selection_cumulative_distr();
    ga.population.update(selected_pop);
    ga.population.inspect();
    println!("{:?}",ga.population.crossover(0,1));
    let mated_pop = ga.population.mate_population();
    ga.population.update(mated_pop);
    ga.population.inspect();
    ga.population.mutate_population_optimization(0.1, -4.0..0.4);
    ga.population.inspect();
    println!("-------------------------");
    let eval = ga.step(-4.0..4.0);
    println!("{:?}",ga.evolve(1000,-4.0..4.0));
    ga.show_fittest();
    let (scores, individuals) = ga.evaluate();
    let inputs = vec![4.,-2.,3.5,5.,-11.,-4.7];
    let solution:f64 = individuals[0].iter().zip(inputs.iter()).map(|(x,y)|x*y).collect::<Vec<f64>>().iter().sum();
    println!("Solution = {}",solution);
}