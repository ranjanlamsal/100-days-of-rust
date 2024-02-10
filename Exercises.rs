fn main(){
    exercise1();

}

//Exercise 1
fn exercise1(){
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

/Exercise 2

fn exercise2() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);
    //using dereference operation to dereference the memory location 
    s
    println!("Success!");
}

// Fix error
fn exercise3() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

fn exercise4(){
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn exercise5(){
    let mut s:String = String::from("hello, ");

    // Fill the blank to make it work
    
    let p : &mut String = &mut s;

    //here since s is a mutable string we can reference to it and change the content
    //using  the pointer p which points to the same object as s and calling
    // pust_str method

    
    p.push_str("world");

    println!("Success!");
}

//ref can be used to take references to a value, similar to &.

//Exercise 6
}

// Get memory address string

fn exercise6() {
    let c: char = '中';

    let r1: &char = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}


//Borrowing Rules

//Exercise 7

// Remove something to make it work
// Don't remove a whole line !
fn exercise7() {
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    //Rule: Can have either only one mutable reference or as many immutable reference as required at a time
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

//Error: Borrow an immutable object as mutable
//Exercise 8

fn Exercise8() {
    // Fix error by modifying this line
    // let  s = String::from("hello, ");
    //Here a immutable variable is accesed through a mutable reference 
    //to solcve this by simply making the s variable a mutable String type

    let  s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

//Ok: Borrow a mutable object as immutable
//Exercise 9

// This code has no errors!
fn Exercise9() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}


//NLL
//Exercise 10

// Comment one line to make it work
fn exercise10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s; //shouldnot use R1 after this point
    r2.push_str("!");
    
    // println!("{}",r1);

    //here two mutable reference pointing to the same variable is not allowed at the same time
    ///however if there are two references to the same variable in a code, then they can be used one by one
    /// i.e first referene must be used before declaring the second reference and after the declaration of
    /// second reference the first reference must not be used

}