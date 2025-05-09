use std::ops::Range;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use std::time::Instant;

pub trait Initialization<T>{
    fn initialize(&self, config: Config) -> T;
}
///struct used to change configuration<BR>
/// there is a Default configuration
#[derive(Clone)]
pub struct Config{
    pub num_individuals:usize,
    pub num_genes:usize,
    pub range:Range<f64>,
    pub mutation_rate:f64,
}
///default configuration
impl Default for Config{
    fn default() -> Self {
        Config{
            num_individuals:10,
            num_genes:10,
            range: -10.0..10.0,//only used in Initialization<Vec<Vec<f64>>>
            mutation_rate:0.1,
        }
    }
}

pub struct RandomInitialization;
pub struct TSPInitialization;
pub struct SudokuInitialization;
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
impl Initialization<Vec<Vec<usize>>> for SudokuInitialization{
    fn initialize(&self, config: Config) -> Vec<Vec<usize>> {
        let mut rng = thread_rng();
        let mut individuals = Vec::with_capacity(config.num_individuals);
        for _ in 0..config.num_individuals{
            let mut individual:Vec<usize> = (0..81).map(|_|rng.gen_range(0..10)).collect();
            //individual.shuffle(&mut rng);
            individuals.push(individual);
        }
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
    pub population: Population,
    fitness:F,
    config: Config,
}
impl<F> GA<F>
where F:Fn(Population)->f64{
    ///initialize population based on an initialization strategy and a fitness function
    pub fn new(initialization:InitializationStrategy, fitness:F, config: Config) -> Self{
        let population = match &initialization {
            InitializationStrategy::Usize(init) =>{
                Population::Usize(init.initialize(config.clone()))
            }
            InitializationStrategy::F64(init) =>{
                Population::F64(init.initialize(config.clone()))
            }
        };
        GA{initialization, population, fitness, config}
    }
    ///print population
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
            Population::Usize(individuals)=>{
                for individual in individuals{
                    eval.push((self.fitness)(Population::Usize(vec![individual.clone()])));
                }
            }
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
            Population::Usize(vec)=> {
                let mut evaluated_individuals: Vec<(Vec<usize>, f64)> = vec.iter().cloned().zip(evals.into_iter()).collect();
                evaluated_individuals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                self.population = Population::Usize(evaluated_individuals.into_iter().map(|(individual, _)| individual).collect(), )
            }
        }
    }
    ///update population with a provided one
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
                Population::F64(selected_parents)
            }
            Population::Usize(vec)=>{
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
                Population::Usize(selected_parents)
            }
        }
    }
    ///shuffle population and perform crossover
    pub fn mate_population(&self)->Population{
        match &self.population {
            Population::F64(vec)=>{
                let mut selected_parents= vec.clone();
                selected_parents.shuffle(& mut rand::thread_rng());
                let mut new_population = Vec::new();
                let selected_parents = Population::F64(selected_parents);
                for i in (0..vec.len()).step_by(2){
                    if i+1 < vec.len(){
                        let (child1, child2) = selected_parents.crossover(i, i+1);
                        new_population.push(child1);
                        new_population.push(child2);
                    }
                }
                Population::F64(new_population)
            }
            Population::Usize(vec)=> {
                let mut selected_parents = vec.clone();
                selected_parents.shuffle(&mut rand::thread_rng());
                let mut new_population = Vec::new();
                let selected_parents = Population::Usize(selected_parents);
                for i in (0..vec.len()).step_by(2) {
                    if i + 1 < vec.len() {
                        let (child1, child2) = selected_parents.crossover(i, i + 1);
                        new_population.push(child1);
                        new_population.push(child2);
                    }
                }
                Population::Usize(new_population)
            }
        }
    }
    ///mutate population
    pub fn mutate(&mut self) ->Population{
        match &self.population {
            Population::F64(vec)=>{
                let mut old_pop = vec.clone();
                let mut rng = rand::thread_rng();
                for i in 0..old_pop.len(){
                    for j in 0..old_pop.first().unwrap().len(){
                        if rng.gen::<f64>() < self.config.mutation_rate{
                            old_pop[i][j]=rng.gen_range(Config::default().range.clone());
                            //println!("Mutating {} gene for {} individual",j,i);
                        }
                    }
                }
                Population::F64(old_pop)
            }
            Population::Usize(vec)=>{
                let mut old_pop = vec.clone();
                let mut rng = rand::thread_rng();
                for i in 0..old_pop.len(){
                    if rng.gen::<f64>() < self.config.mutation_rate{
                        let index1 = rng.gen_range(0..old_pop[i].len());
                        let index2 = rng.gen_range(0..old_pop[i].len());
                        old_pop[i].swap(index1, index2);
                        //println!("Swapping {} and {} genes for individual {}",index1,index2,i);
                    }
                }
                Population::Usize(old_pop)
            }
        }
    }
    ///evolve population forward by one step<BR>
    /// in particular:<BR>
    /// evaluate population<BR>
    /// select population to mate<BR>
    /// mate<BR>
    /// mutate<BR>
    /// evaluate again population<BR>
    /// returns new score of updated population<BR>
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
        evals = self.evaluate();
        print!("... final score = {:?}",evals[0]);
        evals[0].clone()
    }
    ///execute num_steps forward of evolution, return a vector of scores (curve of fitness)
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
pub trait Crossover<T>{
    fn crossover(&self, index1:usize, index2:usize)->(T,T);
}
impl Crossover<Vec<f64>> for Population{
     fn crossover(&self, parent1_index: usize, parent2_index: usize) -> (Vec<f64>,Vec<f64>) {
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
/*
impl Crossover<Vec<usize>> for Population {
    fn crossover_old(&self, parent1_index: usize, parent2_index: usize) -> (Vec<usize>, Vec<usize>) {
        match self{
        Population::Usize(vec)=>{
            let size = vec[parent1_index].len();
            let mut child1 = vec![None; size];
            let mut child2 = vec![None; size];
            let mut rng = rand::thread_rng();
            let start = rng.gen_range(0..size);
            let end = rng.gen_range(start..size);
            // Copy the crossover section from parent1 to child1 and from parent2 to child2
            for i in start..=end {
                child1[i] = Some(vec[parent1_index][i]);
                child2[i] = Some(vec[parent2_index][i]);
            }
            // Fill the rest of child1 from parent2 maintaining the order (with loop back to start of vector)
            let mut current_index1 = (end + 1) % size;
            for &gene in vec[parent2_index].iter() {
//                if !child1.contains(&Some(gene)) {
                if child1[current_index1].is_none(){
                    child1[current_index1] = Some(gene);
                    current_index1 = (current_index1 + 1) % size;
                }
            }
            // Fill the rest of child2 from parent1 maintaining the order (with loop back to start of vector)
            let mut current_index2 = (end + 1) % size;
            for &gene in vec[parent1_index].iter() {
//                if !child2.contains(&Some(gene)) {
                if child2[current_index2].is_none(){
                    child2[current_index2] = Some(gene);
                    current_index2 = (current_index2 + 1) % size;
                }
            }
            (child1.into_iter().map(|x| x.unwrap()).collect(), child2.into_iter().map(|x| x.unwrap()).collect())
        }
            _ => unimplemented!()
        }
    }
}
*/
use std::collections::HashMap;

impl Crossover<Vec<usize>> for Population {
    fn crossover(&self, parent1_index: usize, parent2_index: usize) -> (Vec<usize>, Vec<usize>) {
        match self {
            Population::Usize(vec) => {
                let size = vec[parent1_index].len();
                let mut child1 = vec![None; size];
                let mut child2 = vec![None; size];
                let mut rng = rand::thread_rng();
                let start = rng.gen_range(0..size);
                let end = rng.gen_range(start..size);

                // Crossover Section
                for i in start..=end {
                    child1[i] = Some(vec[parent1_index][i]);
                    child2[i] = Some(vec[parent2_index][i]);
                }

                // Create frequency maps for tracking duplicates
                let mut freq_map_child1 = HashMap::new();
                let mut freq_map_child2 = HashMap::new();

                let mut current_index1 = (end + 1) % size;
                let mut current_index2 = (end + 1) % size;

                for &gene in &vec[parent2_index] {
                    let count_child1 = freq_map_child1.entry(gene).or_insert(0);
                    if !child1[current_index1].is_some() && *count_child1 < count_occurrences(gene, &vec[parent2_index]) {
                        child1[current_index1] = Some(gene);
                        *count_child1 += 1;
                        current_index1 = (current_index1 + 1) % size;
                    }
                }

                for &gene in &vec[parent1_index] {
                    let count_child2 = freq_map_child2.entry(gene).or_insert(0);
                    if !child2[current_index2].is_some() && *count_child2 < count_occurrences(gene, &vec[parent1_index]) {
                        child2[current_index2] = Some(gene);
                        *count_child2 += 1;
                        current_index2 = (current_index2 + 1) % size;
                    }
                }

                (child1.into_iter().map(|x| x.unwrap()).collect(), child2.into_iter().map(|x| x.unwrap()).collect())
            }
            _ => unimplemented!(),
        }
    }
}

// Helper function to count occurrences of a value
fn count_occurrences(value: usize, vec: &Vec<usize>) -> usize {
    vec.iter().filter(|&&x| x == value).count()
}


pub trait GetPopulation<T>{
    fn get_individual(&self, index:usize)->Option<Vec<T>>;
}
impl GetPopulation<f64> for Population{
    fn get_individual(&self, index: usize) -> Option<Vec<f64>> {
        match self {
            Population::F64(vec)=>vec.get(index).cloned(),
            _=>None,
        }
    }
}
impl GetPopulation<usize> for Population{
    fn get_individual(&self, index: usize) -> Option<Vec<usize>> {
        match self {
            Population::Usize(vec)=>vec.get(index).cloned(),
            _=>None,
        }
    }
}