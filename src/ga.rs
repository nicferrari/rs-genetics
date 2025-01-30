use crate::population::{MutatePopulationOptimization, MutatePopulationTSP,Population};
use std::ops::Range;
use std::time::Instant;

pub struct GA<I, F>
where F:Fn(&I)->f64{
    pub population:Population<I>,
    pub fitness: F,
}

impl<I, F> GA <I, F>
where F:Fn(&I)->f64, I: std::clone::Clone + std::fmt::Debug{
    pub fn evaluate(&self) -> (Vec<f64>, Vec<I>) {
        let mut eval = vec![];
        for individual in &self.population.individuals {
            eval.push((self.fitness)(individual));
        }
        let mut paired: Vec<(f64, I)> = eval
            .into_iter()
            .zip(self.population.individuals.clone().into_iter())
            .collect();
        // Sort the individuals based on their evaluation score in descending order
        paired.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let (sorted_scores, sorted_individuals): (Vec<f64>, Vec<I>) = paired.into_iter().unzip();
        (sorted_scores, sorted_individuals)
    }
    pub fn evaluate_individual(&self, index: usize) -> Option<f64> {
        if index < self.population.individuals.len() {
            Some((self.fitness)(&self.population.individuals[index]))
        } else {
            None // Return None if the index is out of bounds
        }
    }
    pub fn show_fittest(&self){
        let (sorted_scores, sorted_individuals) = self.evaluate();
        println!("Highest score = {:?}",sorted_scores[0]);
        println!("Fittest individual = {:?}",sorted_individuals[0]);
    }
}


impl <F> GA <Vec<f64>, F>
where F:Fn(&Vec<f64>)->f64{
    pub fn step(&mut self, range:Range<f64>)->f64 {
        println!("Evolving...");
        let (eval, sorted_pop) = self.evaluate();
        //println!("Best initial fitness = {}...",eval[0]);
        self.population.update(sorted_pop);
        let selected_pop = self.population.rank_selection_cumulative_distr();
        self.population.update(selected_pop);
        let mated_pop = self.population.mate_population();
        self.population.update(mated_pop);
        self.population.mutate_population_optimization(0.1,range);
        let (eval, sorted_pop) = self.evaluate();
        self.population.update(sorted_pop);
        println!("Best final fitness = {}",eval[0]);
        eval[0]
    }
    pub fn evolve(&mut self, num_steps:usize, range: Range<f64>) ->Vec<f64>{
        let start_time = Instant::now();
        let mut hist=Vec::new();
        for i in 0..num_steps{
            println!("Step {}",i);
            hist.push(self.step(range.clone()));
        }
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        println!("Elapsed_time = {:?}",duration);
        hist
    }
}
impl <F> GA <Vec<usize>, F>
where F:Fn(&Vec<usize>)->f64{
    pub fn step(&mut self)->f64 {
        println!("Evolving...");
        let (eval, sorted_pop) = self.evaluate();
        //println!("Best initial fitness = {}...",eval[0]);
        self.population.update(sorted_pop);
        let selected_pop = self.population.rank_selection_cumulative_distr();
        self.population.update(selected_pop);
        let mated_pop = self.population.mate_population();
        self.population.update(mated_pop);
        self.population.mutate_population_tsp(0.1);
        let (eval, sorted_pop) = self.evaluate();
        self.population.update(sorted_pop);
        println!("Best final fitness = {}",eval[0]);
        eval[0]
    }
    pub fn evolve(&mut self, num_steps:usize) ->Vec<f64>{
        let start_time = Instant::now();
        let mut hist=Vec::new();
        for i in 0..num_steps{
            println!("Step {}",i);
            hist.push(self.step());
        }
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        println!("Elapsed_time = {:?}",duration);
        hist
    }
}
