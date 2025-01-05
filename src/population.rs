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
    pub fn evaluate(&self) ->(Vec<f64>, Vec<&Vec<f64>>)
    {
        let mut eval = vec![];
        for i in 0..self.population.individuals.len(){
        let a = (self.evaluate)(&self.population.individuals[i], &self.inputs, self.target);
            println!("Fitness of {} individual is {}",i,a);
            eval.push(a);
        }
        //let mut data = (eval,&self.population.individuals);
        let mut paired: Vec<(f64, &Vec<f64>)> = eval.into_iter().zip(self.population.individuals.iter()).collect();
        //let mut paired = (&eval,&self.population.individuals);
        paired.sort_by(|a,b|a.0.partial_cmp(&b.0).unwrap());
        let (sorted_vec1, sorted_vec2): (Vec<f64>, Vec<&Vec<f64>>) = paired.into_iter().unzip();
        //return (eval,&self.population.individuals)
        return (sorted_vec1,sorted_vec2)
    }
}

pub struct Population{
    pub individuals:Vec<Vec<f64>>,
}

impl Population{
    pub fn new(num_individual:i64)->Self{
        let mut rng = rand::thread_rng();
        //let mut chromosomes:Vec<Vec<f64>>=vec![];
        let mut uniform = Uniform::<f64>::new(-4.,4.);
        let chromosomes = (0..num_individual).map(|_| { (0..6).map(|_| uniform.sample(&mut rng)).collect::<Vec<f64>>() }).collect();
        Population{ individuals: chromosomes }
    }
    pub fn inspect(&self){
        for i in 0.. self.individuals.len(){
            println!("Chromosome = {}",i);
            for j in 0..self.individuals[i].len(){
                print!(" {:.2} ",self.individuals[i][j]);
            }
            println!();
        }
    }
}