fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // here we ARE copying the data, not transfering it

    println!("s1 = {s1}, s2 = {s2}");
}
