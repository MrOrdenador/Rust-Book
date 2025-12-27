fn main() {
    let mut x = 5; // Gotta be "mut" so we can change its value
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Uppercase and snakecase for constants
}
