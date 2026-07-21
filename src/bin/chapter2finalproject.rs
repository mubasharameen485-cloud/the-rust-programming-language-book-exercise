const FREEZING_POINT_F: f64 = 32.0;

fn analyze_tem(celsius: f64) -> (f64, char) {
    let cal = (celsius * 1.8) + 32.0;

    
    let status = if cal <= FREEZING_POINT_F {
        'F'
    } else if cal > 80.0 {
        'H'
    } else {
        'N'
    };

    
    (cal, status)
}

fn main() {
    let temps_celsius: [f64; 5] = [0.0, 40.0, 25.0, -10.0, 15.0];
    
    'system_loop: loop {
        println!("Analyzing weather data...\n"); 
        
        for temp in temps_celsius {
           
           let (f_val, status) = analyze_tem(temp);

           
           if status == 'F' {
               println!("{} degrees F is Freezing! ", f_val);
           } else if status == 'H' {
               println!("{} degrees F is Hot! ", f_val);
           } else if status == 'N' {
               println!("{} degrees F is Normal Weather. ", f_val);
           }
        }
        
        
        break 'system_loop;
    }
}