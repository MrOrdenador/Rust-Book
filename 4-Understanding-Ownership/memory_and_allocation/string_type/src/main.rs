fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!") // appends ", world" to s
    println!("{s}"); // will print hello, world
}
