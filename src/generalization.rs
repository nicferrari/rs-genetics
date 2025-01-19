use rand::Rng;
use rand::seq::SliceRandom;
use std::fmt::Display;
/*pub trait Individual{}

impl Individual for Vec<f64> {}
impl Individual for Vec<usize>{}*/

pub struct Population<I>{
    pub individuals: Vec<Vec<I>>,
}

impl <I> Population<I>{
    pub fn new(individuals: Vec<Vec<I>>) -> Self{
        Population{individuals}
    }
}

impl <I: Display + std::fmt::Debug> Population<I>{
    pub fn inspect(&self) {
        for (index, individual) in self.individuals.iter().enumerate() {
            println!("Individual {}: {:?}", index,individual);
            for (sub_index, element) in individual.iter().enumerate(){
                println!("Element {}: {}", sub_index,element);
            }
        }
    }
}

pub trait PopulationInitialization<I>{
    fn initialize(&self, population_size:usize, num_individuals:usize)->Population<I>;
}

pub struct ContinuousOptimizationInitialization;

impl PopulationInitialization<f64> for ContinuousOptimizationInitialization{
    fn initialize(&self, population_size: usize, num_dimensions: usize) -> Population<f64> {
        let mut rng = rand::thread_rng();
        //TODO: here generates into (-10,10) range - (range: std::ops::Range<i32)
        let individuals: Vec<Vec<f64>> = (0..population_size).map(|_| { (0..num_dimensions).map(|_| rng.gen_range(-10.0..10.0)).collect()}).collect();
        Population{ individuals: individuals }
    }
}
