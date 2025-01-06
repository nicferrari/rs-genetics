use genetic_algorithms::population::GA;

pub fn main() {
/*Given the following function:
y = f(w1:w6) = w1x1 + w2x2 + w3x3 + w4x4 + w5x5 + 6wx6
where (x1,x2,x3,x4,x5,x6)=(4,-2,3.5,5,-11,-4.7) and y=44
What are the best values for the 6 weights (w1 to w6)? We are going to use the genetic algorithm to optimize this function.
*/
    let inputs = vec![4.,-2.,3.5,5.,-11.,-4.7];
    let target = 44.;

    fn fitness(inputs:&Vec<f64>, weights:&Vec<f64>, target:f64)->f64{
        let distance:f64 = inputs.iter().zip(weights.iter()).map(|(x,y)|x*y).collect::<Vec<f64>>().iter().sum();
        return 1./(target-distance).abs();
    }

    let mut ga = GA::new(fitness, 10, inputs, target);
    ga.population.inspect();
    let (eval,sorted_pop) = ga.evaluate();
    for i in 0..eval.len(){
        println!("Individual {} fitness {}",i,eval[i]);
    }
    ga.population.update(sorted_pop.clone());
    ga.population.inspect();

    let new_pop = ga.rank_selection_cumulative_distr();
    ga.population.update(new_pop);
    ga.population.inspect();
}