pub(crate) fn demo_4() {
    let numbers = [10, 20, 30, 40, 50];

    // Trying to get the value at index 2
    match get_location(&numbers, 2) {
        Some(value) => println!("Value at index 2: {}", value),
        None => println!("Index out of bounds"),
    }

    // Trying to get the value at index 10
    match get_location(&numbers, 10) {
        Some(value) => println!("Value at index 10: {}", value),
        
        //2: Comment me out to see what happens if you forget!
        None => println!("Index out of bounds"),
    }
}

fn get_location(arr: &[i32], index: usize) -> Option<&i32> {
    return arr.get(index);
}

// 1: Hover over enum to see what this is.
enum MyOption<T> {
    Some(T),
    None,
}

enum MyResult<T,E> {
    Ok(T),
    Err(E)
}