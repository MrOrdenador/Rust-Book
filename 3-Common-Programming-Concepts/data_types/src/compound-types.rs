fn main() {
    // TUPLES
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // basically indexing
    let six_point_four = x.1;
    let one = x.2;

    // ARRAYS
    let a = [1, 2, 3, 4, 5]; // FIXED SIZE. If we want to change it's size, we gotta use a vector
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]; // Here, it'd be useful to use an array since we know there're always going to be 12 months

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // <- the 5 specifies the max size, and all the values are initialized to 3

    let first = a[0]; // Indexing but with arrays
    let second = a[1]; 
}

