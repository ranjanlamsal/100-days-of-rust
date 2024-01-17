// Functions are declared using the fn keyword.
// Its arguments are type annotated, just like variables, and,
// if the function returns a value, the return type must be specified
//  after an arrow ->.
// The final expression in the function will be used as return value.
//  Alternatively, the return statement can be used to return a value earlier from
//  within the function, even from inside loops or if statements.
// Unlike C/C++, there's no restriction on the order of function definitions

//Diverging function
//A function that never returent to a caller
//example : looping forever, panicing or quiting the program

fn main() {
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);
    prob1();

    prob5();
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
    //the function returns the value by the exporession
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn prob1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    //destructured a tuple to assign to a and b
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success! prob1");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
    //added a return type and removed the semicolon to create a expression
}

// Solve it in two ways
// DON'T let `println!` work
fn prob3() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!(); //panic() will make compiler to panic and exit the code
              //as a result the program will never return to its caller
              //second method would be a recursion call
              //never_return();
              //this would run the block for ever
}

//Diverging functions
// Diverging functions never return to the caller, so they may be used
// in places where a value of any type is expected.

fn prob4() {
    println!("Success! prob4 ");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        // match is just like switch case
        //we can switch to any code block which matches the given parameter
        //return type has Option<i32>
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!(); //panic will make the compiler panic and cause a error
    unimplemented!(); //unimplemented will trap the program. for a function that is not implemented yet
    todo!(); //similar to unimplemented
             //these methods cause the program in trap
}

fn prob5() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success! prob 5");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
