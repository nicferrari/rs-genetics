use std::ops::{Range};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

pub struct Population<I>{
    pub individuals:Vec<I>,
}

pub struct TSP;

//impl Initialize for Population<Vec<usize>>{
impl TSP{
     pub fn initialize(num_individuals:usize, num_genes:usize) -> Population<Vec<usize>> {
         let mut rng = thread_rng();
         let mut individuals = Vec::with_capacity(num_individuals);
         for _ in 0..num_individuals{
             let mut individual:Vec<usize> = (0..num_genes).collect();
             individual.shuffle(&mut rng);
             individuals.push(individual);
         }
         Population{individuals}
    }
}

pub struct Optimization;

impl<I: std::fmt::Debug> Population<I>{
    pub fn inspect(&self){
        for (index, individual) in self.individuals.iter().enumerate(){
            println!("individual {} - {:?}", index,individual);
        }
    }
}

impl<I> Population<I>{
    pub fn update(&mut self, new_population:Vec<I>){
        self.individuals=new_population
    }
}

impl Optimization{
    pub fn initialize(num_individuals: usize, num_genes: usize, range: Range<f64>) -> Population<Vec<f64>> {
        let mut rng = thread_rng();
        let individuals:Vec<Vec<f64>> = (0..num_individuals).map(|_| { (0..num_genes).map(|_| rng.gen_range(range.clone())).collect()}).collect();
        Population{individuals}
    }
}