fn main() {
    let s = String::from("hello, world!");

    let hello = &s[0..5]; // Just slicing such as other langs
    let world = &s[6..1];

    // let slice = &s[0..2]; == let slice = &s[..2];
    let slice = &s[..2];

    // let slice = &s[3..len]; == let slice = &s[3..];
    let slice = &s[3..];

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {word}");
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
