fn main() {
    // Stack par store hone wala integer
    let x: i32 = 100;

    
    let y: Box<i32> = Box::new(200);

    println!("Stack value: {}", x);
    println!("Heap value: {}", y);

    // *y se Heap wali value ko dereference karte hain
    let sum = x + *y;

    println!("Sum: {} + {} = {}", x, *y, sum);
}