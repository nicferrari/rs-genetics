pub trait Initialization<T>{
    fn initialize(&self) -> T;
}
pub struct RandomInitialization;
impl Initialization<Vec<usize>> for RandomInitialization{
    fn initialize(&self) -> Vec<usize> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
}
impl Initialization<Vec<f64>> for RandomInitialization{
    fn initialize(&self) -> Vec<f64> {
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    }
}
pub enum InitializationStrategy{
    Usize(Box<dyn Initialization<Vec<usize>>>),
    F64(Box<dyn Initialization<Vec<f64>>>),
}
pub struct GA{
    initialization: InitializationStrategy,
}
impl GA{
    pub fn new(initialization:InitializationStrategy) -> Self{
        GA{initialization}
    }
    pub fn run(&self){
        match (&self.initialization) {
            InitializationStrategy::Usize(init) =>{
                let mut population = init.initialize();
                println!("usize!");
            }
            InitializationStrategy::F64(init) =>{
                let mut population = init.initialize();
                println!("f64!")
            }
        }
    }
}

