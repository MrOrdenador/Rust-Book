fn main() {
    let s = "hello";
    { // not valid here, since it's not been declared yet
        let s = "hello"; // valid from here
    } // s is not valid anymore
}
