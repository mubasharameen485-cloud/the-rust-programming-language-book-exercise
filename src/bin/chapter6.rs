// 1. Enum Definition
enum Food {
    Pizza,
    Burger,
    Juice(String), /
    Other,
}


fn get_price(item: Food) -> Option<u32> {
    match item {
        Food::Pizza => Some(500),
        Food::Burger => Some(300),
        
        
        Food::Juice(flavor) => {
            println!("Juice flavor: {}", flavor);
            Some(150)
        }
        
        
        _ => None,
    }
}

fn main() {
    
    let my_order = Food::Juice(String::from("Apple"));
    let order_price = get_price(my_order);

    
    match order_price {
        Some(price) => println!("Aapka bill hai: {} Rs\n", price),
        None => println!("Sorry, item available nahi hai!\n"),
    }

    /
    let invalid_order = Food::Other;
    let invalid_price = get_price(invalid_order);

    match invalid_price {
        Some(price) => println!("Aapka bill hai: {} Rs", price),
        None => println!("Sorry, item available nahi hai!"),
    }
}