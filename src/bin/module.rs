```rust
mod student {
    pub fn name() {
        println!("My name is Ali");
    }

    fn age() {
        println!("My age is 20");
    }
}

fn main() {
    student::name();

    // student::age(); // Error: age private hai
}
```
