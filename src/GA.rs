use crate::population::Population;

pub struct GA<I, F>
where F:Fn(&I)->f64{
    pub population:Population<I>,
    pub fitness: F,
}

impl<I, F> GA <I, F>
where F:Fn(&I)->f64, I: std::clone::Clone{
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


}

/*
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
}*/