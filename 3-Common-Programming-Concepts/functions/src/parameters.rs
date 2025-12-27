/* 
fn main() {
    another_function();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
*/

// another example

fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}