//Statement:
//Instruction that performs some action but do not produce a value
//Function definations are statements, as well as code that ends with ";"(usually)
//
//Expressions:
///Evaluate to return a value
///
///
fn main() {
    let x: u32 = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        //this exression will be assigned to y

        x_cube + x_squared + x
    };
    // here assignment of y is a statement and the code block for assignment
    //of y is an expression as it returns the value to be the value of y

    let z = {
        2 * x;
        //here ";" at the end means that this is a statement and thus () unit value is assigned to z
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    prob1();
    prob2();
    prob3();
}

fn prob1() {
    // Make it work with two ways
    let v = {
        let mut x = 1;

        //x += 2 it is a assignment as it means x = x+2
        //so it must end with an ;

        x += 2;
        //since an statement is used in assignment of v, v would have an unit () value
    };

    //one way would be assertion with a unity as assignment of v is with a unit value
    assert_eq!(v, ());

    //another way would be assigning v with a expression such that v holds 3 as to
    //assert_eq!(v, 3); would result in success

    println!("Success! prob1");
}

fn prob2() {
    // let v = (let x = 3);
    ///here let x = 3 is a statement and hence should end with a semicolon
    ///and hence v would hold an unit value
    let v = {
        let x = 3; //statement
        x //expression such that v is assigned 3
    }; //this is also an statement so end with an ;

    assert!(v == 3);

    println!("Success! prob2");
}

fn prob3() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success! prob3");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y //no semicolon as it is an expression and we need to return a i32 type value
}
