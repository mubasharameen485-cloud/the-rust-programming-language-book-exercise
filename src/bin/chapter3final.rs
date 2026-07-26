fn main() {
    
    let full_name = String::from("ALI RAZA"); 
    
    let first_name = get_first_name(&full_name);
    
    println!("Your first name is: {}", first_name);
}


fn get_first_name(name: &str) -> &str {
    let bytes = name.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &name[0..i];
        } 
    }

   
    &name[..]
}