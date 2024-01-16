//In Rust Tuples can be represented using parentheses.
//A tuple is a collection of values of different types.
//Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members.
// Functions can use tuples to return multiple values, as tuples can hold any number of values.

//tuples as function arguments and return value

fn sum(x: i32, y: i32) -> i32 {
    let sum = x + y;
    sum
}

fn main() {
    let x: i32 = 5;
    let y: i32 = 10;
    let addition: i32 = sum(x, y);
    println!("{}", addition);

    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    //printing a tuple
    let pair = (1, true);
    println!("Pair is {:?}", pair);

    //1 element tuple needs and appending comma
    let one_element_tuple = (1,);
    println!("{:?}", one_element_tuple);

    //tuples can be destructured to create bindings

    let tuple = ("element1", 2i32, 3.0f32);

    let (a, b, c) = tuple;

    println!("{},{},{}", a, b, c);
}
