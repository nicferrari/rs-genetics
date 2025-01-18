use genetic_algorithms::generalize::{Initialize, Population};

fn main() {
    let mut pop_usize:Population<Vec<usize>> = Population::initialize(2,5);
    pop_usize.inspect();
    let mut pop_f64:Population<Vec<f64>> = Population::initialize(2,5);
    pop_f64.inspect();
    let pop_f64_range:Population<Vec<f64>> = Population::initialize_with_range(2,5,-1.0..1.0);
    pop_f64_range.inspect();
    pop_usize.update(vec![vec![1,2],vec![2,3]]);
    pop_usize.inspect();
    pop_f64.update(vec![vec![0.4,-0.1],vec![3.2,2.5]]);
    pop_f64.inspect();
}