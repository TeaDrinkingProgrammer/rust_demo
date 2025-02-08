pub(crate) fn demo_1() {
    let name = String::from("World");
    say_hello_to(name);
    // store_to_db(name);
}

fn say_hello_to(name: String) {
    println!("{}", name);
}

fn store_to_db(name: String) {
    println!("User {} is stored to db", name);
}