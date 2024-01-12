#[allow(unused_variables)]
//this would not show the warning "unused variable"

//Assigned using 'let' keyword
//Print to stabdard iutput by 'print!()' or 'println!()'
//Scope of a cariable defined by the block of code in which it is declared
//Function is a named block of code that is reusable
//Shadowing allows a variable to be re-declared in the same scope with the same name




// A variable can be used only if it has been initialized with a value
fn main() {
    //1 Declare and initialize a variable
    let x: i32 = 5; //Initialized and used, SUCCESS

    // let y : i32; // Uninitialized but also unused, only a Warning!
    //let z : i32; //Uninitialized but used, ERROR !
    
    //assert_eq(A,B) takes two parameters and then equates the parameter
    //If A and B are not equat the program gives runtime error
    //assert_eq!(z, 5); // Here z is not initialized but is used.

    assert_eq!(x,5);
    println!("SUCCESS");

    //We can use "_" to used unintialized variables for the program to not throw warning
    //Example

    let _z:i32; //Uninitialized and unused But doesnot shows warning


    //2
    //Use mut to mark a variable as mutable.

    let mut a = 1; //mut keyword is used if the variable is to be mutable
    //Here type is not assigned but that is a bad practice

    a = a+2;

    assert_eq!(a, 3);
    println!("SUCCESS");


    //3
    //Scope : A scope is the range within the program for which the item is valid.
    // Fix the error below with least amount of modification

    let _b: i32 = 10;
    {
        let _c: i32 = 5;
        println!("The value of b is {} and value of c is {}", _b, _c);
        //Here c is defined in this scope and b is in the outer scope. Soo the code works fine
    }
    // println!("The value of b is {} and value of c is {}", b,c);
    //This will give an error because c is out of its Scope. It was declared inside
    let _c : i32 = 4;
    println!("The value of b is {} and value of c is {}", _b,_c); //Here b and c are defined in this scope


// 4 Fix the error with the use of define_x

    // println!("{}, world", x);
    //Error Here because x is not in the same scope
    // we need to call define_x function 
    define_x();

    shadowing_fn();

    compile_fn();

    unused();

    destructure_fn();

    destructure_assignment();

}
fn define_x() {
    let _x = "hello";
    println!("{}, world", _x);
}

//5 Shadowing
// You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.

fn shadowing_fn() {
    let x: i32 = 5;
    {
        let x = 12;
        // assert_eq!(x, 5); //Throws runtime error as x is again defined in this scope
        assert_eq!(x,12)
    }

    //assert_eq!(x, 12);
    // error as x is searched first in the main scope and then only in the inner scope
    // since in the main scope the value of x is 5 this throws runtime-error
    assert_eq!(x,5); //No error
    let x = 42; //shadowed again
    println!("{}", x); // Prints "42".
}

//6 
// Remove a line in the code to make it compile
fn compile_fn() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x; 
    // Here as x is reinitialized as immutable(default) it shouldnot be change later
    x += 3;


    let _y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 
    //to remove warning use _y as it is never used
    //Assign a type to y as &str as its value is changed from int to str


    println!("Success!");
    println!("{}",y);
}

//Unused variables
//7. Fix the warning below with :
// 🌟 Only one solution
// 🌟🌟 Two distinct solutions
#[allow(unused_variables)]
//should be used in every function block where unused variables and declared
fn unused() {
    let x = 1; 
    //method one is to prefix variable name with "_"
    let _x = 1;

    //method 2 would be using #[allow(unused_variables)]
    //method 2 tells compiler to allow the unused variables and wont show any warning

}

// Warning: unused variable: `x`

//8 Destructuring
// We can use a pattern with let to destructure a tuple to separate variables.
//Tips: you can use Shadowing or Mutability

// Fix the error below with least amount of modification
#[allow(unused_variables)]
fn destructure_fn(){
    // let mut (X,y) = (1,2);  //This is incorrect
    //`mut` must be attached to each individual binding
    // help: add `mut` to each binding: `(mut x, mut y)`

    let (x, y) = (1, 2);
    let(mut x, y) = (1,2);
    //tuple of x and y are destructured using let keyword
    x += 2;//x is mutated although it is immutable.
    //Need to define x of tuple as mutable

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

//let (x,y); is same as:
//let x;
//let y;

// 9. Destructuring assignments
// Introduced in Rust 1.59: You can now use tuple, slice, and struct patterns as the left-hand side of an assignment
fn destructure_assignment() {
    let (x, y); //declaring x and y at the same time
    //destructuring assignment
    (x,..) = (3, 4); //This assigns the valus x to 3 but doesnot care about the next variable
    [.., y] = [1, 2];//This assigns the valus y to 2 but doesnot care about the first variable
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);
    //since x is destructured as 3 and y as 2

    println!("Success!");
} 