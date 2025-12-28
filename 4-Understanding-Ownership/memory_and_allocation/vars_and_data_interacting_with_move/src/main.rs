fn main() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello"); // A string is made of: a pointer to the memory, a length and a capacity. Stored on the Stack
    let s2 = s1; // Here we copy the three parts, we are pointing to the same things, we are NOT making new ones

    // println!("{s1}, world!"); error because the s1 was moved into s2, getting invalidated.
}
