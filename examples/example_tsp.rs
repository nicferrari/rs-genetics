// solving the traveling salesman problem with the library
pub fn main(){
    struct City{x:f64,y:f64,};
    let cities = vec![City{x:0.,y:0.},City{x:1.,y:1.},];
    struct Tour{cities:Vec<usize>,}
    fn distance_(city1: &City, city2: &City) -> f64 {
        ((city1.x - city2.x).powi(2) + (city1.y - city2.y).powi(2)).sqrt()
    }
    impl Tour {
        fn total_distance(&self, cities: &Vec<City>) -> f64 {
            let mut distance = 0.0;
            for i in 0..self.cities.len() - 1 {
                distance += distance_(&cities[self.cities[i]], &cities[self.cities[i + 1]]);
            }
            distance += distance_(&cities[self.cities[self.cities.len() - 1]], &cities[self.cities[0]]);
            distance
        }
    }
    let tour = Tour{cities:vec![0,1]};
    println!("{}",tour.total_distance(&cities));

}