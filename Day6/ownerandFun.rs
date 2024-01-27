fn main() {
    let s = String::from("hello");
    //s is defined in main scope

    takes_ownership(s); //s's value moves into the takes_ownership function...
                        //and so is no longer valid here

    // println!("{}", &s); //error is thrown
    let x = 5; //x comes into the scope
    make_copy(x);
    //Here x would move into the function but since it is i32 type
    //it would be copied into the stack rather than moved
    //it is legal to use x afterwards
    println!("{}", x);

    let s1 = gives_ownership(); //gives_ownership moves its return value to s1

    let s2 = String::from("hello"); //s2 comes into scope

    let s3 = takes_and_give_back(s2); //s2 is moved into take_and_give_back,
                                      //which also moves its return value into s3
}
//here s3 and s1 are moved out of the scope and are dropped
//here s2 is moved so nothing happens

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
//here some_string goes out of the scope and 'drop' is called. The backing memory is freed

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
//Here some_integer goes out of scope, Nothing special Happens

fn gives_ownership() -> String {
    //gives ownership will move its return value into the function that calls it

    let new_string = String::from("yours");
    //new_string comes into the scope

    new_string //new_string is returned and moves out to the calling function
}

fn takes_and_give_back(a_string: String) -> String {
    //a_string comes into the scope
    a_string //s_string is returned and moves out to the calling function
}

fn prob1() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("Hello world");
    let y: String = x.clone();
    println!("{}, {}", x, y);
}

fn prob2() {
    // Don't modify code in main!
    let s1 = String::from("Hello world");
    let s2: String = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn prob3() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String = String::from("Hello world");
    // Convert String to Vec
    // let _s = s.into_bytes();
    // Convert Vec<u8> to String
    let _s = s.as_bytes();
    //Here into_byte functions consumes the variable s
    //asbyte function rather takes reference and the value is only changed without the variable being consumed
    s
}

fn prob4() {
    // Fix the error without removing any code
    let s: String = String::from("Hello World");

    print_str(s.clone());
    //variable is now owned by the fdunction print_str and cannot be further used

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s)
}

fn prob5() {
    // Don't use clone ,use copy instead
    // let x = (1, 2, (), "hello".to_string());
    //here one of the element is of type string which cannot be simply copied rather the ownership needs to be trasfered

    //instead using a &str type which is  just a reference to string data in heap

    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    //since x is all static type it can be simply copied
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}

// make the necessary variable mutable
fn prob6() {
    let s: String = String::from("Hello ");

    //Here the variable after the ownership has been transfered is mutated using a mut keyword
    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

fn prob7() {
    let x: Box<i32> = Box::new(5);

    let mut y: Box<i32> = Box::new(1); // update this line, don't change other lines!

    //dereference y and assign a value
    *y = 4;

    assert_eq!(*x, 5);
    //*x would be the value stored in the location pointed by x

    println!("Success!");
}

fn prob8() {
    let t = (String::from("hello"), String::from("world"));

    let _s: String = t.0;
    //_s is now the owner of the data t.0
    //so we cannot access the t.0
    //we also cannot access the t as a whole as it only partially own its variable

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}

fn prob9() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Fill the blanks
    //destructuring the tuple intop s1 and s2
    let (s1, s2) = t.clone();
    //here s1 and s2 will get the copy of the t.0 and t.1 values
    //the ownership is not transfered and we can still use the t variable

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
