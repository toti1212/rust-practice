struct CarPool {
    passengers: Vec<String>,
}

impl CarPool {

    fn pool(&mut self, passenger: String) {
        self.passengers.push(passenger)
    }
}


fn main() {
    let mut c1 = CarPool {
        passengers: vec![String::from("Rust")]
    };

    println!("Car 1 passenges: {:?}", c1.passengers);    
    c1.pool(String::from("Rodrigo"));
    println!("Car 1 passenges: {:?}", c1.passengers);

}

