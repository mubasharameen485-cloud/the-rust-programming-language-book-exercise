fn main() {
    let arr = [10, 25, 7, 45, 18];
    let mut largest = arr[0];

    for i in 1..arr.len() {
        if arr[i] > largest {
            largest = arr[i];
        }
    }

    println!("Largest number = {}", largest);
}