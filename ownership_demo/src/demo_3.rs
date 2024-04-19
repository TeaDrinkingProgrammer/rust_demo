pub(crate) fn demo_3() {
    let numbers = [10, 20, 30, 40, 50];
    // Trying to get the value at index 2
    //try {
    let value = get_location(&numbers, 2);
    //} catch
    println!("Value at index 2: {}", value);

    // Trying to get the value at index 10
    let value = get_location(&numbers, 10);
    println!("Help: my program crashed!");
    println!("Value at index 10: {}", value);
}

fn get_location(arr: &[i32], index: usize) -> i32 {
    if index < arr.len() {
        arr[index]
    } else {
        panic!("Oh no, something went wrong")
    }
}