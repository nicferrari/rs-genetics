use crate::population::Population;

pub struct GA<'a, I, F>
where F:Fn(&I)->f64{
    pub population:Population<I>,
    pub fitness: &'a F,
}

impl<'a, I, F> GA <'a, I, F>
where F:Fn(&I)->f64{
    pub fn evaluate(&self)->f64{
        println!("#indiv = {}",self.population.individuals.len());
        (self.fitness)(self.population.individuals.first().unwrap())
    }
}


pub struct LinearCombination{
    weights: Vec<f64>,
    xs: Vec<f64>,
    target:f64,
}

pub trait FitnessFunction{
    fn calculate_fitness(&self)->f64;
}
impl FitnessFunction for LinearCombination{
    fn calculate_fitness(&self) -> f64 {
        let result:f64 = self.xs.iter().zip(self.weights.iter()).map(|(x,w)|x*w).sum();
        (self.target-result).abs()
    }
}