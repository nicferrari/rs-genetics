use std::ops::Index;
use rand_distr::{Uniform, Distribution};

pub struct GA<F>
where F: Fn(&Vec<f64>, &Vec<f64>, f64)->f64
{
    pub population: Population,
    evaluate: F,
    pub target: f64,
    pub inputs: Vec<f64>,
}

impl <F> GA<F>
where F: Fn(&Vec<f64>, &Vec<f64>, f64)->f64
{
    pub fn new(evaluate: F, num_individuals: i64, inputs:Vec<f64>, target:f64)->Self{
        let population = self::Population::new(num_individuals);
        GA{population, evaluate, target, inputs}
    }
    pub fn evaluate(&self) ->(Vec<f64>, Vec<Vec<f64>>)
    {
        let mut eval = vec![];
        for i in 0..self.population.individuals.len(){
        let a = (self.evaluate)(&self.population.individuals[i], &self.inputs, self.target);
            //println!("Fitness of {} individual is {}",i,a);
            eval.push(a);
        }
        //let mut data = (eval,&self.population.individuals);
        let mut paired: Vec<(f64, &Vec<f64>)> = eval.into_iter().zip(self.population.individuals.iter()).collect();
        //let mut paired = (&eval,&self.population.individuals);
        paired.sort_by(|a,b|a.0.partial_cmp(&b.0).unwrap());
        let (sorted_vec1, sorted_vec2): (Vec<f64>, Vec<&Vec<f64>>) = paired.into_iter().unzip();
        let sorted_vec2_:Vec<Vec<f64>> = sorted_vec2.into_iter().map(|v|v.clone()).collect();
        //return (eval,&self.population.individuals)
        return (sorted_vec1,sorted_vec2_)
    }
    pub fn rank_selection_single_prob(&self) -> Vec<Vec<f64>>{
        let total_rank:f64 = (1..=self.population.individuals.len()).map(|i|i as f64).sum();
        let mut selected_parents = Vec::new();
        while selected_parents.len()<self.population.individuals.len(){
            for (rank, individual) in self.population.individuals.iter().enumerate(){
                let rank_prob = (self.population.individuals.len() - rank) as f64 / total_rank;
                if rand::random::<f64>() < rank_prob {
                    println!("selecting {}",rank);
                    selected_parents.push(individual.clone());
                    if selected_parents.len()==self.population.individuals.len(){break}
                };
            }
        }
        selected_parents
    }
    pub fn rank_selection_cumulative_distr(&self) -> Vec<Vec<f64>>{
        let total_rank:f64 = (1..=self.population.individuals.len()).map(|i|i as f64).sum();
        let mut selected_parents = Vec::with_capacity(self.population.individuals.len());
        let mut sum=0.;
        let mut cumulative_prob = Vec::with_capacity(self.population.individuals.len());
        for (rank, individual) in self.population.individuals.iter().enumerate() {
            sum += (self.population.individuals.len() - rank) as f64 / total_rank;
            cumulative_prob.push((sum, individual.clone()));
        }
        for _ in 0..self.population.individuals.len() {
            let rand_num: f64 = rand::random::<f64>();
            for &(prob, ref individual) in &cumulative_prob {
                if rand_num < prob {
                    println!("Selecting with {} {}",rand_num, prob);
                    selected_parents.push(individual.clone());
                    break;
                }
            }
        }
        selected_parents
    }
}

pub struct Population{
    pub individuals:Vec<Vec<f64>>,
}

impl Population{
    pub fn new(num_individual:i64)->Self{
        let mut rng = rand::thread_rng();
        let uniform = Uniform::<f64>::new(-4.,4.);
        let chromosomes = (0..num_individual).map(|_| { (0..6).map(|_| uniform.sample(&mut rng)).collect::<Vec<f64>>() }).collect();
        Population{ individuals: chromosomes }
    }
    ///Shows the weights of the individuals
    pub fn inspect(&self){
        for i in 0.. self.individuals.len(){
            println!("Individual = {}",i);
            for j in 0..self.individuals[i].len(){
                print!(" {:.2} ",self.individuals[i][j]);
            }
            println!();
        }
    }
    pub fn update(& mut self, new_population: Vec<Vec<f64>>){
        self.individuals = new_population;
    }
}