/*

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is {x}");
}


*/

// Another example

fn main() {
    let x = plus_one(5);

    println!("The value of is {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon, it's not a statement!
}
