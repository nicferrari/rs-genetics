use genetic_algorithms::population::{Initialize, Optimization, Population, TSP};

fn main() {
    /*
    let mut pop_usize:Population<Vec<usize>> = Population::initialize(2,5);
    pop_usize.inspect();
    let mut pop_f64:Population<Vec<f64>> = Population::initialize(2,5);
    pop_f64.inspect();
    let pop_f64_range:Population<Vec<f64>> = Population::initialize_with_range(2,5,-1.0..1.0);
    pop_f64_range.inspect();
    pop_usize.update(vec![vec![1,2],vec![2,3]]);
    pop_usize.inspect();
    pop_f64.update(vec![vec![0.4,-0.1],vec![3.2,2.5]]);
    pop_f64.inspect();*/
    let tsp_population = TSP::initialize(2,5);
    tsp_population.inspect();
    //let mut optimization_population = Optimization::initialize(2,5);
    //optimization_population.inspect();
    let mut optimization_population = Optimization::initialize(2,5,0.0..1.0);
    optimization_population.inspect();
    optimization_population.update(vec![vec![0.4,-0.1],vec![3.2,2.5]]);
    optimization_population.inspect();
}