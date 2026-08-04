fn main() {
    // Allocate integer on the heap
    let num = Box::new(100);

    println!("Value = {}", num);

    // Memory is automatically freed when `num` goes out of scope.
}