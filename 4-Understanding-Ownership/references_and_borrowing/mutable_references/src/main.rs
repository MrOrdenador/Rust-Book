fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
    // println!("{r1} and {r2} again") // ERROR because s's got now a mutable reference

}

fn change(some_string: &mut String) {
    some_string.push_str(", world"); // Here we are changing s' value, without passing a reference before it
}

