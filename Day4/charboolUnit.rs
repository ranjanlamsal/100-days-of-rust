//Char type variable can hold 4 byte value i.e 1 character

use std::mem::size_of_val;
//size_of_val is a attribute of any variable that gives the size it takes in memory.
//a char takes 4 bytw of memory while a i32 takes 32 byte

fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '中';
    println!("'{}'", size_of_val(&c2));

    let a = "String"; // double quote means its a string
    let b: char = 'q'; //single quote -> character

    //Boolean
    boo1();

    //Unit Type
    // empty tuple of size 0 byte
    //A type that doesnot hold any value. Its size is 0 byte
    //if a functioon doesnot return any value, then a unit type is returned implicitly
    unitType();

    let unit: () = ();
    println!("Size of Unit is {:?}", size_of_val(&unit));
}

// Make println! work
fn boo1() -> () {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}

fn unitType() {
    // Make it work, don't modify `implicitly_ret_unit` !

    let _v: () = ();
    //_v is defined as a unit type value as it doesnot hold anything

    let v = (2, 3);
    //v is defined as a tuple

    assert_eq!(_v, implicitly_ret_unit());
    //implicitly_ret_unit() function doesnot return anything thus will return a unit yype

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// This function is explicitly defined to return a unit type
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
