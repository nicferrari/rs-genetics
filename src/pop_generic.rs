use std::ops::Range;
use std::process::Output;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use crate::pop_generic::Population::F64;
use std::time::Instant;

pub trait Initialization<T>{
    fn initialize(&self, config: Config) -> T;
}
///struct used to change configuration
/// there is a Default configuration
pub struct Config{
    num_individuals:usize,
    num_genes:usize,
    range:Range<f64>,
    mutation_rate:f64,
}
///default configuration
impl Default for Config{
    fn default() -> Self {
        Config{
            num_individuals:100,
            num_genes:10,
            range: -10.0..10.0,
            mutation_rate:0.1,
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
impl Population{
    pub fn get_individual_at(&self, index:usize)->Option<Vec<f64>>{
        match self {
            Population::F64(vec) => vec.get(index).cloned(),
            _ => None,
        }
    }
    pub fn get_individuals(&self)->Option<Vec<Vec<f64>>>{
        match self {
            Population::F64(vec) => Some(vec.clone()),
            _ => None,
        }
    }
}


pub struct GA<F>
where F:Fn(Population)->f64{
    initialization: InitializationStrategy,
    pub population: Population,
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
    ///sort in place population based on an input vector of fitness
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
    ///update population from a provided one
    pub fn update(&mut self, new_population:Population){
        self.population = new_population
    }
    ///rank select parents based on cumulative distribution function
    pub fn rank_selection_cdf(&self)->Population{
        match &self.population {
            Population::F64(vec)=>{
                let total_rank: f64 = (1..=vec.len()).map(|i| i as f64).sum();
                let mut selected_parents = Vec::with_capacity(vec.len());
                let mut sum = 0.0;
                let mut cumulative_prob = Vec::with_capacity(vec.len());
                for (rank, individual) in vec.iter().enumerate() {
                    sum += (vec.len() - rank) as f64 / total_rank;
                    cumulative_prob.push((sum, individual.clone()));  // Clone the individual
                }
                for _ in 0..vec.len() {
                    let rand_num: f64 = rand::random::<f64>();
                    for &(prob, ref individual) in &cumulative_prob {
                        if rand_num < prob {
                            // Adding cloned individual to the selected parents
                            selected_parents.push(individual.clone());
                            break;
                        }
                    }
                }
                F64(selected_parents)
            }
            _ => unimplemented!(),
        }
    }
    ///shuffle population and perform crossover
    pub fn mate_population(&self)->Population{
        match &self.population {
            F64(vec)=>{
                let mut selected_parents= vec.clone();
                selected_parents.shuffle(& mut rand::thread_rng());
                let mut new_population = Vec::new();
                let mut selected_parents = Population::F64(selected_parents);
                for i in (0..vec.len()).step_by(2){
                    if i+1 < vec.len(){
                        let (child1, child2) = selected_parents.crossover(i, i+1);
                        new_population.push(child1);
                        new_population.push(child2);
                    }
                }
                F64(new_population)
            }
            _ => unimplemented!(),
        }
    }
    pub fn mutate(&mut self) ->Population{
        match &self.population {
            F64(vec)=>{
                let mut old_pop = vec.clone();
                let mut rng = rand::thread_rng();
                for i in 0..old_pop.len(){
                    for j in 0..old_pop.first().unwrap().len(){
                        if rng.gen::<f64>() < Config::default().mutation_rate{
                            old_pop[i][j]=rng.gen_range(Config::default().range.clone());
                            //println!("Mutating {} gene for {} individual",j,i);
                        }
                    }
                }
                Population::F64(old_pop)
            }
            _ => unimplemented!(),
        }
    }
    pub fn step(&mut self)->f64{
        let mut evals = self.evaluate();
        self.sort(evals.clone());
        print!("Initial score = {} .... evolving ...",evals[0].clone());
        let selected = self.rank_selection_cdf();
        self.update(selected);
        let mated_pop = self.mate_population();
        self.update(mated_pop);
        let mutated_pop = self.mutate();
        self.update(mutated_pop);
        evals = self.evaluate();
        self.sort(evals.clone());
        print!("... final score = {:?}",evals[0]);
        evals[0].clone()
    }
    pub fn evolve(&mut self, num_steps:usize) ->Vec<f64>{
        let start_time = Instant::now();
        let mut hist=Vec::new();
        for i in 0..num_steps{
            println!();
            println!("Step {}",i);
            hist.push(self.step());
        }
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        println!();
        println!("Elapsed_time = {:?}",duration);
        hist
    }
}
pub trait Crossover{
    type Output;
    fn crossover(&self, index1:usize, index2:usize)->(Self::Output, Self::Output);
}
impl Crossover for Population{
    type Output = Vec<f64>;
     fn crossover(&self, parent1_index: usize, parent2_index: usize) -> (Self::Output, Self::Output) {
        match &self{
            Population::F64(vec)=>{
                let gene_length = vec[parent1_index].len();
                let crossover_point = rand::random::<usize>() % gene_length;
                let mut child1_genes = vec[parent1_index][..crossover_point].to_vec();
                child1_genes.extend_from_slice(&vec[parent2_index][crossover_point..]);
                let mut child2_genes = vec[parent2_index][..crossover_point].to_vec();
                child2_genes.extend_from_slice(&vec[parent1_index][crossover_point..]);
                (child1_genes,child2_genes)
            }
            _ => unimplemented!()
        }
    }
}
