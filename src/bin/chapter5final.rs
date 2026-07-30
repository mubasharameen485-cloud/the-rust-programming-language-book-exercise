#[derive(Debug)]
struct Car {
    name: String,
    top_speed: u32,
}

impl Car {
    
    fn new_car(car_name: &str, speed: u32) -> Car {
        Car {
            name: String::from(car_name),
            top_speed: speed, 
        }
    } 

    
    fn is_fast(&self) -> bool {
        if self.top_speed > 150 {
            true 
        } else {
            false
        }
    }
} 

fn main() {
    
    let my_car = Car::new_car("Honda", 180);

    let lee = my_car.is_fast();
    println!("Is my car fast? {}", lee);
    
    dbg!(&my_car);
}