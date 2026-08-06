fn main() {
    // Stack variable
    let x: i32 = 100;

    // Heap variable
    let y = Box::new(200);

    println!("Stack value: {}", x);
    println!("Heap value: {}", y);

    let z = *y + x;
    println!("Sum = {}", z);
}