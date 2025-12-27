fn main() {
    let number = 3; // second example with number = 7, which returns false

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number {
        println!("number was three"); // <- will return an error because number isn't a bool val
    }

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn handling_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");  // Yeah, the same as JS w/o ()s
    }
}

fn ternaries() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // love this :), although I prefer JS' ternary
            // if we return "six", it will give us an error because the func is expected to return only a type
    println!("The value of number is: {number}");
}