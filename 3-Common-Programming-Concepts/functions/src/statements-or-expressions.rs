fn main() {
    // let y = 6; // statement -> we are not returning anything
    // let x = (let y = 6); // error -> because let y = 6 is a statement, not an expression

    let y = {
        let x = 3;
        x + 1 // expression -> no semicolon
    };
}
