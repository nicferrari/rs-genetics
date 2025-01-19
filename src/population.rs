use std::time::Instant;
use rand::Rng;
use rand::seq::SliceRandom;

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
    ///initializes GA
    pub fn new(evaluate: F, num_individuals: usize, inputs:Vec<f64>, target:f64)->Self{
        println!("Initializing population of {} individuals",num_individuals);
        let num_genes = inputs.len();
        let population = self::Population::new(num_individuals, num_genes);
        GA{population, evaluate, target, inputs}
    }
    ///evaluates population based on the function provided during initialization
    pub fn evaluate(&self) ->(Vec<f64>, Vec<Vec<f64>>)
    {
        println!("Evaluating population");
        let mut eval = vec![];
        for i in 0..self.population.individuals.len(){
        let a = (self.evaluate)(&self.population.individuals[i], &self.inputs, self.target);
            //println!("Fitness of {} individual is {}",i,a);
            eval.push(a);
        }
        //let mut data = (eval,&self.population.individuals);
        let mut paired: Vec<(f64, &Vec<f64>)> = eval.into_iter().zip(self.population.individuals.iter()).collect();
        //let mut paired = (&eval,&self.population.individuals);
        //TODO: the below is to maximize target function; to implement a flexible solution to switch cases
        paired.sort_by(|a,b|b.0.partial_cmp(&a.0).unwrap());
        let (sorted_vec1, sorted_vec2): (Vec<f64>, Vec<&Vec<f64>>) = paired.into_iter().unzip();
        let sorted_vec2_:Vec<Vec<f64>> = sorted_vec2.into_iter().map(|v|v.clone()).collect();
        //return (eval,&self.population.individuals)
        return (sorted_vec1,sorted_vec2_)
    }
    ///rank select using a single probability for each element
    pub fn rank_selection_single_prob(&self) -> Vec<Vec<f64>>{
        println!("Rank selecting by single probability");
        let total_rank:f64 = (1..=self.population.individuals.len()).map(|i|i as f64).sum();
        let mut selected_parents = Vec::new();
        while selected_parents.len()<self.population.individuals.len(){
            for (rank, individual) in self.population.individuals.iter().enumerate(){
                let rank_prob = (self.population.individuals.len() - rank) as f64 / total_rank;
                if rand::random::<f64>() < rank_prob {
                    //println!("selecting {}",rank);
                    selected_parents.push(individual.clone());
                    if selected_parents.len()==self.population.individuals.len(){break}
                };
            }
        }
        selected_parents
    }
    ///rank select using a cumulative distribution (one extraction produces one selected individuals)
    pub fn rank_selection_cumulative_distr(&self) -> Vec<Vec<f64>>{
        println!("Rank selecting by cumulative distribution");
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
                    //println!("Selecting with {} {}",rand_num, prob);
                    selected_parents.push(individual.clone());
                    break;
                }
            }
        }
        selected_parents
    }
    ///mates selected population via crossover
    pub fn mate_population(&self) -> Vec<Vec<f64>> {
        println!("Mating population");
        let selected_parents = &mut self.population.individuals.clone();
        selected_parents.shuffle(& mut rand::thread_rng());
        let mut new_population = Vec::new();
        for i in (0..self.population.individuals.len()).step_by(2){
            if i+1 < selected_parents.len(){
                let (child1, child2) = crossover(&selected_parents[i], &selected_parents[i+1]);
                new_population.push(child1);
                new_population.push(child2);
            }
        }
        new_population
    }
    ///mutate population based on a mutation rate
    pub fn mutate_population(&mut self, mutation_rate:f64){
        let mut rng = rand::thread_rng();
        for i in 0..self.population.individuals.len(){
            for j in 0..self.population.individuals.first().unwrap().len(){
                if rng.gen::<f64>() < mutation_rate{
                    self.population.individuals[i][j]=rng.gen_range(-4.0..4.0);
                    //println!("Mutating {} gene for {} individual",i,j);
                }
            }
        }
    }
    ///one step of evolution (i.e. calculation of initial fitness, selection of parents, crossover, mutation and recalculation of fitness)
    pub fn step(&mut self) -> f64{
        println!("Evolving...");
        let (eval, sorted_pop) = self.evaluate();
        //println!("Best initial fitness = {}...",eval[0]);
        self.population.update(sorted_pop);
        let selected_pop = self.rank_selection_cumulative_distr();
        self.population.update(selected_pop);
        let mated_pop = self.mate_population();
        self.population.update(mated_pop);
        self.mutate_population(0.1);
        let (eval, sorted_pop) = self.evaluate();
        self.population.update(sorted_pop);
        println!("Best final fitness = {}",eval[0]);
        eval[0]
    }
    ///performs a define number of steps
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

pub struct Population{
    pub individuals:Vec<Vec<f64>>,
}

impl Population{
    pub fn new(num_individuals:usize, num_genes:usize) ->Self{
        let mut rng = rand::thread_rng();
        //TODO: here generates into (-10,10) range - (range: std::ops::Range<i32)
        let chromosomes: Vec<Vec<f64>> = (0..num_individuals).map(|_| { (0..num_genes).map(|_| rng.gen_range(-10.0..10.0)).collect()}).collect();
        Population{ individuals: chromosomes }
    }
    ///shows the weights of the individuals
    pub fn inspect(&self){
        for i in 0.. self.individuals.len(){
            println!("Individual = {}",i);
            for j in 0..self.individuals[i].len(){
                print!(" {:.2} ",self.individuals[i][j]);
            }
            println!();
        }
    }
    ///updates population with a provided one
    pub fn update(& mut self, new_population: Vec<Vec<f64>>){
        //println!("Updating population");
        self.individuals = new_population;
    }
}
///performs random single-point crossover (i.e. crossover_point is randomly selected vs possible fixed point)
pub fn crossover(parent1:&Vec<f64>, parent2:&Vec<f64>)->(Vec<f64>,Vec<f64>){
    let gene_length = parent1.len();
    let crossover_point = rand::random::<usize>() % gene_length;
    let mut child1_genes = parent1[..crossover_point].to_vec();
    child1_genes.extend_from_slice(&parent2[crossover_point..]);
    let mut child2_genes = parent2[..crossover_point].to_vec();
    child2_genes.extend_from_slice(&parent1[crossover_point..]);
    (child1_genes,child2_genes)
}