use rand_distr::num_traits::Float;
use genetic_algorithms::population_specific::GA;

pub fn main() {
    /*Given the following function:
      finds the maximum of the function f(x) = 10-(x+3)^2 (which is (-3,10))
    */
    let inputs = vec![1.];
    let target = 10.;

    fn fitness(weights:&Vec<f64>, inputs:&Vec<f64>, target:f64)->f64{
        //let distance:f64 = inputs.iter().zip(weights.iter()).map(|(x,y)|x*y).collect::<Vec<f64>>().iter().sum();
        return target-(weights[0]+3.).powi(2);
    }

    let mut ga = GA::new(fitness, 10, inputs.clone(), target);

    let hist = ga.evolve(1000);
    for i in 0..hist.len(){
        println!("Epoch {} fitness = {:.5}",i,hist[i]);
    }
    let solution:f64 = 10.-(ga.population.individuals[0][0]+3.).powi(2);
    println!("Solution = {} vs target {}",solution,target);
}