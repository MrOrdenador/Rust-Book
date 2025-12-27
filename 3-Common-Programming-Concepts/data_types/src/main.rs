fn main() {

    // let guess = "42".parse().expect("Not a number!"); // Errror because .parse() needs a type annotation

    let guess: : u32 = "42".parse().expect("Not a number!"); // Now it works

    /*

       DATA TYPES

    ┌───────────────────────┬────────┬──────────┐
    │ Length                │ Signed │ Unsigned │
    ├───────────────────────┼────────┼──────────┤
    │ 8-bit                 │ i8     │ u8       │
    │ 16-bit                │ i16    │ u16      │
    │ 32-bit                │ i32    │ u32      │
    │ 64-bit                │ i64    │ u64      │
    │ 128-bit               │ i128   │ u128     │
    │ Architecture-dependent│ isize  │ usize    │
    └───────────────────────┴────────┴──────────┘

    ┌──────────────────────┬───────────────┐
    │ Number literals      │ Example       │
    ├──────────────────────┼───────────────┤
    │ Decimal              │ 98_222        │
    │ Hex                  │ 0xff          │
    │ Octal                │ 0o77          │
    │ Binary               │ 0b1111_0000   │
    │ Byte (u8 only)       │ b'A'          │
    └──────────────────────┴───────────────┘

    */
    
    /*
        If using the --release flag:
        The code doesn't panic, and if the int goes higher than (i/u)k
        the limits, it goes to [noutofthelimits] mod(2^k)
    */

}
