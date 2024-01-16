//Integer

//Integers can be represented by wither signed or unsigned representation
// i8 -> 8 bit signed integer
//i16 -> 16 but signed integer
//i32 -> 32 bit signed integer
//isize -> n bit signed integer (n = bit size of the architecture)

//Integers can be represented by wither signed or unsigned representation
// u8 -> 8 bit unsigned integ -> 16 but unsigned integer
//u32 -> 32 bit unsigned integer
//usize -> n bit unsigned integer (n = bit size of the architecture)

fn main() {
    let var1 = 42;
    let var2 = "Hello, Rust!";

    // Using std::any::type_name to get the type as a string

    println!("Type of var1: {}", type_of(&var1)); //by default i32
                                                  //if you want any other type rather than i32 , it must be explicitly expressed

    println!("Type of var2: {}", type_of(&var2));

    let _v: u16 = 38_u8 as u16;
    //38_u8 means that 38 is value of type u8
    //but we are assigning a value of u8 to a variable of type u16
    //to do this we should use 'as' keyword
    //as keyword can take a value of type as a value of different type

    println!("Success!");

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("SUCCESS!");
    //i8 means integer of size 8 bits. SO its max value would be 127
    //u8 means unsigned integer of size 8 bits and its max value would be 255

    // let v1 = 251_u8 + 8;
    //Here compiler would treat the code as
    // let v1::u8 = 251_u8 + 8_u8
    //but the value of v1 would be 259 which would exceed the
    //MAX value that a u_8 could hold that is 255

    //In Rust you can perform mathematical calculation among different number systems

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    println!("{}", v);

    //Floating point numbers
    let x = 1_000.000_1; //by default f64
    let y = 0.12; //by default f32

    println!(
        "Type of 1000.0001 is {} and 0.12 is {}",
        type_of(&x),
        type_of(&y)
    );

    // assert_eq!(0.1 + 0.2 == 0.3);
    //above line generates error as the floating point expressions are too precise
    //to solve declare type within the expression
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    //using as keyword
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);

    range_fn();
}

fn type_of<T>(_: &T) -> String {
    format!("{:?}", std::any::type_name::<T>())
}

//Range
///Range can be used to iterate over a range of values
/// for i in -3..2{} it means that the code inside the block
/// would run for i as every value between -3 and 2
/// -3 is included but 2 is not
///
/// to include 2 ,
/// for i in -3..=2{}
///
use std::ops::{Range, RangeInclusive};
fn range_fn() -> () {
    let mut sum: i32 = 0;
    for i in -3..2 {
        //i ranges from -3,-2, -1, 0, 1
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        //c ranges from a and includes z too

        println!("{}", c);

        println!("{}", c as u8);
        // here we have take c as u8 characters which would be converted according to character's respective ASCII values
    }

    //std::ops::Range and RangeInclusive

    //1..5 is similar to Range{start : 1, end:5 }
    //1..=5 is similar to RangeInclusive::new(1,5)

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}
