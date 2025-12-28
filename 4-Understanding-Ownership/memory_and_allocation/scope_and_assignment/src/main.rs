fn main() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // we are just dropping the "hello" 

    println!("{s}, world")
}
