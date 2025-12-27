fn main() {
    let x = 5;

    let x = x + 1; // we are SHADOWING the previous x

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len(); // SHADOWING allows us to change the type

    let mut spaces = "    ";
    spaces = spaces.len(); // This would cause an error because we are changing the type of a mutable variable
}
