// Literals and operators
// Integers 1, floats 1.2, characters 'a', strings "abc", booleans true and the unit type () can be expressed using literals.

// Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: 0x, 0o or 0b.

// Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000, and 0.000_001 is the same as 0.000001.

// Rust also supports scientific E-notation, e.g. 1e6, 7.6e-4. The associated type is f64.

// We need to tell the compiler the type of the literals we use. For now, we'll use the u32 suffix to indicate that the literal is an unsigned 32-bit integer, and the i32 suffix to indicate that it's a signed 32-bit integer.

// The operators available and their precedence in Rust are similar to other C-like languages.

fn main() {
    //integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    //here 1u32 specifies that we are using 1 as a unsigned number

    //Integer sub
    println!("1 - 2 = {}", 1i32 - 2);
    //note : we cannot subtract higher values from lower in unsigned
    //(iu32 - 2) would throw error 'attempt to compute 1u32 - 2u32 which would overflow'

    //Scientific Notation
    println!("1e4 is {}", 1e4);

    //Boolean algebra
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    //Bitwise operation
    // binary representation : 0b(binary value)
    //ocatal representation: 0o(octal value)
    //hex representation : 0x(hexadecimal value)

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    //Readability: large numbers can be represented by stuffing _ in between different digits

    println!("large number: {}", 1_0_0_0_0_0_0_0u32); // result : large number : 1000000
}
