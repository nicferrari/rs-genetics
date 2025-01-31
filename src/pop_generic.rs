use std::ops::Range;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

pub trait Initialization<T>{
    fn initialize(&self, config: Config) -> T;
}
///struct used to change configuration
/// there is a Default configuration
pub struct Config{
    num_individuals:usize,
    num_genes:usize,
    range:Range<f64>,
}
///default configuration
impl Default for Config{
    fn default() -> Self {
        Config{
            num_individuals:10,
            num_genes:10,
            range: -10.0..10.0,
        }
    }
}

pub struct RandomInitialization;
pub struct TSPInitialization;
impl Initialization<Vec<Vec<usize>>> for TSPInitialization{
    fn initialize(&self, config: Config) -> Vec<Vec<usize>> {
        let mut rng = thread_rng();
        let mut individuals = Vec::with_capacity(config.num_individuals);
        for _ in 0..config.num_individuals{
            let mut individual:Vec<usize> = (0..config.num_genes).collect();
            individual.shuffle(&mut rng);
            individuals.push(individual);
        }
        individuals
    }
}
impl Initialization<Vec<Vec<f64>>> for RandomInitialization{
    fn initialize(&self, config: Config) -> Vec<Vec<f64>> {
        let mut rng = thread_rng();
        let individuals:Vec<Vec<f64>> = (0..config.num_individuals).map(|_| { (0..config.num_genes).map(|_| rng.gen_range(config.range.clone())).collect()}).collect();
        individuals
    }
}
pub enum InitializationStrategy{
    Usize(Box<dyn Initialization<Vec<Vec<usize>>>>),
    F64(Box<dyn Initialization<Vec<Vec<f64>>>>),
}
#[derive(Debug)]
pub enum Population{
    Usize(Vec<Vec<usize>>),
    F64(Vec<Vec<f64>>),
}
pub struct GA<F>
where F:Fn(Population)->f64{
    initialization: InitializationStrategy,
    population: Population,
    fitness:F,
}
impl<F> GA<F>
where F:Fn(Population)->f64{
    ///initialize population based on an initialization strategy and a fitness function
    pub fn new(initialization:InitializationStrategy, fitness:F) -> Self{
        let population = match &initialization {
            InitializationStrategy::Usize(init) =>{
                Population::Usize(init.initialize(Config::default()))
            }
            InitializationStrategy::F64(init) =>{
                Population::F64(init.initialize(Config::default()))
            }
        };
        GA{initialization, population, fitness}
    }
    pub fn inspect(&self){
        println!("{:?}",self.population);
    }
    ///evaluate fitness of the whole population
    pub fn evaluate(&self)->Vec<f64>{
        let mut eval = vec![];
        match &self.population {
            Population::F64(individuals)=>{
                for individual in individuals{
                    eval.push((self.fitness)(Population::F64(vec![individual.clone()])));
                }
            }
            _ =>{},
        }
        eval
    }
    ///sort population based on a input vector of fitness
    pub fn sort(&mut self, evals:Vec<f64>){
        match &self.population {
            Population::F64(vec)=>{
                let mut evaluated_individuals:Vec<(Vec<f64>,f64)> = vec.iter().cloned().zip(evals.into_iter()).collect();
                evaluated_individuals.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
                self.population = Population::F64(evaluated_individuals.into_iter().map(|(individual,_)|individual).collect(),)
            }
            _ => {}
        }
    }
}
