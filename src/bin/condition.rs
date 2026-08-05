fn main() {
    let arr = [4, 7, 2, 9, 5, 2, 8];

    let mut found = false;

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] == arr[j] {
                println!("First Duplicate Element: {}", arr[i]);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    if !found {
        println!("No Duplicate Element Found");
    }
}