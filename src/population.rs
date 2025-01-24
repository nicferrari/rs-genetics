use std::ops::{Index, Range};
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
    ///rank select using a cumulative distribution (one extraction produces one selected individuals)
    pub fn rank_selection_cumulative_distr(&self) -> Vec<I>
    where
        I: Clone,  // Ensure the individuals can be cloned
    {
        println!("Rank selecting by cumulative distribution");
        let total_rank: f64 = (1..=self.individuals.len()).map(|i| i as f64).sum();
        let mut selected_parents = Vec::with_capacity(self.individuals.len());
        let mut sum = 0.0;
        let mut cumulative_prob = Vec::with_capacity(self.individuals.len());

        for (rank, individual) in self.individuals.iter().enumerate() {
            sum += (self.individuals.len() - rank) as f64 / total_rank;
            cumulative_prob.push((sum, individual.clone()));  // Clone the individual
        }
        for _ in 0..self.individuals.len() {
            let rand_num: f64 = rand::random::<f64>();
            for &(prob, ref individual) in &cumulative_prob {
                if rand_num < prob {
                    // Adding cloned individual to the selected parents
                    selected_parents.push(individual.clone());
                    break;
                }
            }
        }
        selected_parents
    }
    pub fn mate_population(&self) -> Vec<I>
    where Self:Crossover<I>,
        I:Clone,
    {
        println!("Mating population");
        let mut selected_parents:Vec<I> = self.individuals.clone();
        selected_parents.shuffle(& mut rand::thread_rng());
        let mut new_population = Vec::new();
        let mut selected_parents:Population<I> = Population{individuals:selected_parents};
        for i in (0..self.individuals.len()).step_by(2){
            if i+1 < selected_parents.individuals.len(){
                let (child1, child2) = selected_parents.crossover(i, i+1);
                new_population.push(child1);
                new_population.push(child2);
            }
        }
        new_population
    }

}

impl Optimization{
    pub fn initialize(num_individuals: usize, num_genes: usize, range: Range<f64>) -> Population<Vec<f64>> {
        let mut rng = thread_rng();
        let individuals:Vec<Vec<f64>> = (0..num_individuals).map(|_| { (0..num_genes).map(|_| rng.gen_range(range.clone())).collect()}).collect();
        Population{individuals}
    }
}

pub trait Crossover<I>{
    fn crossover(&self, parent1_index:usize, parent2_index:usize)->(I,I);
}

impl Crossover<Vec<f64>> for Population<Vec<f64>>{
///performs random single-point crossover (i.e. crossover_point is randomly selected vs possible fixed point) for optimization problems
    fn crossover(&self, parent1_index:usize, parent2_index:usize)->(Vec<f64>,Vec<f64>){
        let gene_length = self.individuals[parent1_index].len();
        let crossover_point = rand::random::<usize>() % gene_length;
        let mut child1_genes = self.individuals[parent1_index][..crossover_point].to_vec();
        child1_genes.extend_from_slice(&self.individuals[parent2_index][crossover_point..]);
        let mut child2_genes = self.individuals[parent2_index][..crossover_point].to_vec();
        child2_genes.extend_from_slice(&self.individuals[parent1_index][crossover_point..]);
        (child1_genes,child2_genes)
    }
}
impl Crossover<Vec<usize>> for Population<Vec<usize>>{
    ///performs order crossover for TSP
    fn crossover(&self, parent1_index: usize, parent2_index: usize) -> (Vec<usize>, Vec<usize>) {
        let size = self.individuals[parent1_index].len();
        let mut child1 = vec![None; size];
        let mut child2 = vec![None; size];
        let mut rng = rand::thread_rng();
        let start = rng.gen_range(0..size);
        let end = rng.gen_range(start..size);
        // Copy the crossover section from parent1 to child1 and from parent2 to child2
        for i in start..=end {
            child1[i] = Some(self.individuals[parent1_index][i]);
            child2[i] = Some(self.individuals[parent2_index][i]);
        }
        // Fill the rest of child1 from parent2 maintaining the order
        let mut current_index1 = (end + 1) % size;
        for &gene in self.individuals[parent2_index].iter() {
            if !child1.contains(&Some(gene)) {
                child1[current_index1] = Some(gene);
                current_index1 = (current_index1 + 1) % size;
            }
        }
        // Fill the rest of child2 from parent1 maintaining the order
        let mut current_index2 = (end + 1) % size;
        for &gene in self.individuals[parent1_index].iter() {
            if !child2.contains(&Some(gene)) {
                child2[current_index2] = Some(gene);
                current_index2 = (current_index2 + 1) % size;
            }
        }
        (child1.into_iter().map(|x| x.unwrap()).collect(), child2.into_iter().map(|x| x.unwrap()).collect())
    }
}
pub trait MutatePopulationOptimization<I>{
    fn mutate_population_optimization(&mut self, mutation_rate:f64, range:Range<f64>);
}
impl MutatePopulationOptimization<f64> for Population<Vec<f64>> {
    fn mutate_population_optimization(&mut self, mutation_rate:f64, range: Range<f64>){
        let mut rng = rand::thread_rng();
        for i in 0..self.individuals.len(){
            for j in 0..self.individuals.first().unwrap().len(){
                if rng.gen::<f64>() < mutation_rate{
                    self.individuals[i][j]=rng.gen_range(-4.0..4.0);
                    //println!("Mutating {} gene for {} individual",j,i);
                }
            }
        }
    }
}
pub trait MutatePopulationTSP<I>{
    fn mutate_population_tsp(&mut self, mutation_rate:f64);
}
impl MutatePopulationTSP<usize> for Population<Vec<usize>>{
    fn mutate_population_tsp(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.individuals.len(){
                if rng.gen::<f64>() < mutation_rate{
                        let index1 = rng.gen_range(0..self.individuals[i].len());
                        let index2 = rng.gen_range(0..self.individuals[i].len());
                        self.individuals.swap(index1, index2);
                        println!("Swapping {} and {} genes for individual {}",index1,index2,i);
                }
        }
    }
}