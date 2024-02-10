fn main() {
    let s1 = String::from("Hello");

    let len = calculate_len(&s1);
    //Since the pointer to the original variable is used we are borrowing instead of passing the ownership

    println!("The length of the '{}' is {}", s1, len);
    //SInce s1 is just borrowed, it can be further accessed in the program .
    //Recall that if s1 wasn't reused by using a pointer instead directly
    //by the function name. it would have transferred its ownership and couldnot be used

    //Mutable References
    let mut s2 = String::from("hello");

    change(&mut s);
    //here the change function is taking a reference of 2 and since we are going to alter the value
    // hold by s , we need to explicitly call it by using a variable "mut"
    //It is called as Mutable reference

    //RULE
    //We can have olny one mutable reference to the same data at a time

    let mut s = String::from("Hello!");
    // let r1 = &mut s;
    // let r2 = &mut s;
    //above statement would voilate the rule as there are two variables trying to access the
    //same mutable variable

    //Loophole to have many mutable reference to the same variable

    {
        let r1 = &mut s;
    } // r1 goes out of the scope here, so we can make a new reference with no problem
    let r2 = &mut s;

    let r3 = &s; //no problem as r2 is used as immutable reference of s.
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    //some_string as a formal parameter takes a mutable reference
    some_string.push_str(", world!");
}
