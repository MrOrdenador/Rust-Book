fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // We are NOT taking s1s value out of scope

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
